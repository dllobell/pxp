<?php

use Illuminate\Support\Str;
use Symfony\Component\Yaml\Yaml;

require_once __DIR__ . '/../vendor/autoload.php';

$ast = Yaml::parseFile(__DIR__ . '/../../crates/pxp-ast/meta/ast.yaml');
$output = <<<'RUST'
#![allow(unreachable_code, unreachable_patterns)]
// This file is generated by meta/scripts/generate-ast.php.
// Do not make modifications to this file directly.

use crate::utils::CommaSeparated;
use pxp_syntax::comments::{CommentGroup, Comment};
use pxp_type::Type;
use pxp_token::Token;
use pxp_span::{Span, Spanned};
use pxp_symbol::Symbol;
use pxp_syntax::backed_enum_type::BackedEnumType;
use pxp_syntax::name::NameQualification;
use super::Nodeable;


RUST;

$reserved = ['as', 'derive', 'node'];

function is_spanned(string $node, array $structure): bool {
    if (isset($structure['span'])) {
        return true;
    }

    if ($node === 'StatementKind' || $node === 'ExpressionKind') {
        return false;
    }

    foreach ($structure as $field => $value) {
        if (is_array($value) && isset($value['span'])) {
            return true;
        }

        if (is_string($value) && $value === 'Span') {
            return true;
        }
    }

    return false;
}

foreach ($ast as $node => $structure) {
    if (is_string($structure)) {
        $output .= "pub type {$node} = {$structure};\n\n";
        continue;
    }

    $derive = 'Debug, PartialEq, Eq, Clone';

    if (is_array($structure) && isset($structure['derive'])) {
        $derive .= ', ' . $structure['derive'];
    }

    $output .= "#[derive({$derive})]\n";
    $enum = isset($structure['as']) && $structure['as'] === 'Enum';

    if ($enum) {
        $output .= "pub enum {$node} {\n";
    } else {
        $output .= "pub struct {$node} {\n";
    }

    if ($enum) {
        foreach ($structure as $field => $value) {
            if (in_array($field, $reserved, true)) {
                continue;
            }

            $output .= "    {$field}";

            if ($value === '') {
                $output .= ",\n";
            } elseif (is_string($value)) {
                $output .= "({$value}),\n";
            } elseif (is_array($value)) {
                $output .= " {\n";

                foreach ($value as $subfield => $subtype) {
                    $output .= "        {$subfield}: {$subtype},\n";
                }

                $output .= "    },\n";
            }
        }
    } else {
        $output .= "    pub id: NodeId,\n";
        
        foreach ($structure as $field => $type) {
            if (in_array($field, $reserved, true)) {
                continue;
            }

            $output .= "    pub {$field}: {$type},\n";
        }
    }

    $output .= "}\n\n";

    if (!isset($structure['node'])) {
        $output .= "impl Nodeable for {$node} {\n";
        $output .= "    fn as_node(&self) -> Node {\n";
        $output .= "        Node::{$node}(self)\n";
        $output .= "    }\n";
        $output .= "}\n\n";
    }

    if (! is_spanned($node, $structure)) {
        continue;
    }

    $output .= "impl Spanned for {$node} {\n";
    $output .= "    fn span(&self) -> Span {\n";
    
    if (isset($structure['span'])) {
        $output .= "        self.span";
    } elseif ($enum) {
        $output .= "        match self {";
        foreach ($structure as $field => $value) {
            if (is_string($value) && $value === 'Span') {
                $output .= "{$node}::{$field}(span) => *span,\n";
            } elseif (is_array($value)) {
                $output .= "{$node}::{$field} { span, .. } => *span,";
            }
        }
        $output .= "_ => Span::default(),\n";
        $output .= "        }";
    }

    $output .= "    }";
    $output .= "}\n\n";
}

$output .= "#[derive(Debug, PartialEq, Clone)]\n";
$output .= "pub enum Node<'a> {\n";

foreach ($ast as $node => $structure) {
    if ($node === 'NodeId') {
        continue;
    }

    if (isset($structure['node']) && $structure['node'] === false) {
        continue;
    }

    $output .= "    {$node}(&'a {$node}),\n";
}

$output .= "}\n\n";

$output .= "impl<'a> Node<'a> {\n";

foreach ($ast as $node => $structure) {
    if ($node === 'NodeId') {
        continue;
    }

    if (isset($structure['node']) && $structure['node'] === false) {
        continue;
    }

    $kebab = strtolower(Str::snake($node));

    $output .= "    pub fn as_{$kebab}(self) -> Option<&'a {$node}> {\n";
    $output .= "        match self {\n";
    $output .= "            Node::{$node}(node) => Some(node),\n";
    $output .= "            _ => None,\n";
    $output .= "        }\n";
    $output .= "    }\n\n";

    $output .= "    pub fn is_{$kebab}(&self) -> bool {\n";
    $output .= "        matches!(self, Node::{$node}(_))\n";
    $output .= "    }\n\n";
}

$output .= "}\n";

file_put_contents(__DIR__ . '/../../crates/pxp-ast/src/generated.rs', $output);

echo "AST file generated.\n";