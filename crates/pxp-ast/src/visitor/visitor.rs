// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.
#![allow(unused)]

use super::*;
use crate::*;
use pxp_span::Span;
use pxp_type::Type;

pub trait Visitor {
    fn visit(&mut self, node: &Vec<Statement>) {
        walk(self, node);
    }

    fn visit_statement(&mut self, node: &Statement) {
        walk_statement(self, node);
    }

    fn visit_statement_kind(&mut self, node: &StatementKind) {
        walk_statement_kind(self, node);
    }

    fn visit_expression(&mut self, node: &Expression) {
        walk_expression(self, node);
    }

    fn visit_expression_kind(&mut self, node: &ExpressionKind) {
        walk_expression_kind(self, node);
    }

    fn visit_missing_expression(&mut self, node: &MissingExpression) {}

    fn visit_static_expression(&mut self, node: &StaticExpression) {}

    fn visit_self_expression(&mut self, node: &SelfExpression) {}

    fn visit_parent_expression(&mut self, node: &ParentExpression) {}

    fn visit_comment_statement(&mut self, node: &CommentStatement) {}

    fn visit_inline_html_statement(&mut self, node: &InlineHtmlStatement) {}

    fn visit_full_opening_tag_statement(&mut self, node: &FullOpeningTagStatement) {}

    fn visit_short_opening_tag_statement(&mut self, node: &ShortOpeningTagStatement) {}

    fn visit_echo_opening_tag_statement(&mut self, node: &EchoOpeningTagStatement) {}

    fn visit_closing_tag_statement(&mut self, node: &ClosingTagStatement) {}

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) {
        walk_expression_statement(self, node);
    }

    fn visit_global_statement(&mut self, node: &GlobalStatement) {
        walk_global_statement(self, node);
    }

    fn visit_block_statement(&mut self, node: &BlockStatement) {
        walk_block_statement(self, node);
    }

    fn visit_cast_kind(&mut self, node: &CastKind) {}

    fn visit_case(&mut self, node: &Case) {
        walk_case(self, node);
    }

    fn visit_use(&mut self, node: &Use) {
        walk_use(self, node);
    }

    fn visit_use_kind(&mut self, node: &UseKind) {
        walk_use_kind(self, node);
    }

    fn visit_eval_expression(&mut self, node: &EvalExpression) {
        walk_eval_expression(self, node);
    }

    fn visit_empty_expression(&mut self, node: &EmptyExpression) {
        walk_empty_expression(self, node);
    }

    fn visit_die_expression(&mut self, node: &DieExpression) {
        walk_die_expression(self, node);
    }

    fn visit_exit_expression(&mut self, node: &ExitExpression) {
        walk_exit_expression(self, node);
    }

    fn visit_isset_expression(&mut self, node: &IssetExpression) {
        walk_isset_expression(self, node);
    }

    fn visit_unset_expression(&mut self, node: &UnsetExpression) {
        walk_unset_expression(self, node);
    }

    fn visit_print_expression(&mut self, node: &PrintExpression) {
        walk_print_expression(self, node);
    }

    fn visit_concat_expression(&mut self, node: &ConcatExpression) {
        walk_concat_expression(self, node);
    }

    fn visit_instanceof_expression(&mut self, node: &InstanceofExpression) {
        walk_instanceof_expression(self, node);
    }

    fn visit_reference_expression(&mut self, node: &ReferenceExpression) {
        walk_reference_expression(self, node);
    }

    fn visit_parenthesized_expression(&mut self, node: &ParenthesizedExpression) {
        walk_parenthesized_expression(self, node);
    }

    fn visit_error_suppress_expression(&mut self, node: &ErrorSuppressExpression) {
        walk_error_suppress_expression(self, node);
    }

    fn visit_include_expression(&mut self, node: &IncludeExpression) {
        walk_include_expression(self, node);
    }

    fn visit_include_once_expression(&mut self, node: &IncludeOnceExpression) {
        walk_include_once_expression(self, node);
    }

    fn visit_require_expression(&mut self, node: &RequireExpression) {
        walk_require_expression(self, node);
    }

    fn visit_require_once_expression(&mut self, node: &RequireOnceExpression) {
        walk_require_once_expression(self, node);
    }

    fn visit_function_call_expression(&mut self, node: &FunctionCallExpression) {
        walk_function_call_expression(self, node);
    }

    fn visit_function_closure_creation_expression(
        &mut self,
        node: &FunctionClosureCreationExpression,
    ) {
        walk_function_closure_creation_expression(self, node);
    }

    fn visit_method_call_expression(&mut self, node: &MethodCallExpression) {
        walk_method_call_expression(self, node);
    }

    fn visit_method_closure_creation_expression(&mut self, node: &MethodClosureCreationExpression) {
        walk_method_closure_creation_expression(self, node);
    }

    fn visit_nullsafe_method_call_expression(&mut self, node: &NullsafeMethodCallExpression) {
        walk_nullsafe_method_call_expression(self, node);
    }

    fn visit_static_method_call_expression(&mut self, node: &StaticMethodCallExpression) {
        walk_static_method_call_expression(self, node);
    }

    fn visit_static_variable_method_call_expression(
        &mut self,
        node: &StaticVariableMethodCallExpression,
    ) {
        walk_static_variable_method_call_expression(self, node);
    }

    fn visit_static_method_closure_creation_expression(
        &mut self,
        node: &StaticMethodClosureCreationExpression,
    ) {
        walk_static_method_closure_creation_expression(self, node);
    }

    fn visit_static_variable_method_closure_creation_expression(
        &mut self,
        node: &StaticVariableMethodClosureCreationExpression,
    ) {
        walk_static_variable_method_closure_creation_expression(self, node);
    }

    fn visit_property_fetch_expression(&mut self, node: &PropertyFetchExpression) {
        walk_property_fetch_expression(self, node);
    }

    fn visit_nullsafe_property_fetch_expression(&mut self, node: &NullsafePropertyFetchExpression) {
        walk_nullsafe_property_fetch_expression(self, node);
    }

    fn visit_static_property_fetch_expression(&mut self, node: &StaticPropertyFetchExpression) {
        walk_static_property_fetch_expression(self, node);
    }

    fn visit_constant_fetch_expression(&mut self, node: &ConstantFetchExpression) {
        walk_constant_fetch_expression(self, node);
    }

    fn visit_short_array_expression(&mut self, node: &ShortArrayExpression) {
        walk_short_array_expression(self, node);
    }

    fn visit_array_expression(&mut self, node: &ArrayExpression) {
        walk_array_expression(self, node);
    }

    fn visit_list_expression(&mut self, node: &ListExpression) {
        walk_list_expression(self, node);
    }

    fn visit_new_expression(&mut self, node: &NewExpression) {
        walk_new_expression(self, node);
    }

    fn visit_interpolated_string_expression(&mut self, node: &InterpolatedStringExpression) {
        walk_interpolated_string_expression(self, node);
    }

    fn visit_heredoc_expression(&mut self, node: &HeredocExpression) {
        walk_heredoc_expression(self, node);
    }

    fn visit_nowdoc_expression(&mut self, node: &NowdocExpression) {}

    fn visit_shell_exec_expression(&mut self, node: &ShellExecExpression) {
        walk_shell_exec_expression(self, node);
    }

    fn visit_bool_expression(&mut self, node: &BoolExpression) {}

    fn visit_array_index_expression(&mut self, node: &ArrayIndexExpression) {
        walk_array_index_expression(self, node);
    }

    fn visit_short_ternary_expression(&mut self, node: &ShortTernaryExpression) {
        walk_short_ternary_expression(self, node);
    }

    fn visit_ternary_expression(&mut self, node: &TernaryExpression) {
        walk_ternary_expression(self, node);
    }

    fn visit_coalesce_expression(&mut self, node: &CoalesceExpression) {
        walk_coalesce_expression(self, node);
    }

    fn visit_clone_expression(&mut self, node: &CloneExpression) {
        walk_clone_expression(self, node);
    }

    fn visit_match_expression(&mut self, node: &MatchExpression) {
        walk_match_expression(self, node);
    }

    fn visit_throw_expression(&mut self, node: &ThrowExpression) {
        walk_throw_expression(self, node);
    }

    fn visit_yield_expression(&mut self, node: &YieldExpression) {
        walk_yield_expression(self, node);
    }

    fn visit_yield_from_expression(&mut self, node: &YieldFromExpression) {
        walk_yield_from_expression(self, node);
    }

    fn visit_cast_expression(&mut self, node: &CastExpression) {
        walk_cast_expression(self, node);
    }

    fn visit_default_match_arm(&mut self, node: &DefaultMatchArm) {
        walk_default_match_arm(self, node);
    }

    fn visit_match_arm(&mut self, node: &MatchArm) {
        walk_match_arm(self, node);
    }

    fn visit_magic_constant_expression(&mut self, node: &MagicConstantExpression) {
        walk_magic_constant_expression(self, node);
    }

    fn visit_magic_constant_kind(&mut self, node: &MagicConstantKind) {
        walk_magic_constant_kind(self, node);
    }

    fn visit_string_part(&mut self, node: &StringPart) {
        walk_string_part(self, node);
    }

    fn visit_literal_string_part(&mut self, node: &LiteralStringPart) {}

    fn visit_expression_string_part(&mut self, node: &ExpressionStringPart) {
        walk_expression_string_part(self, node);
    }

    fn visit_array_item(&mut self, node: &ArrayItem) {
        walk_array_item(self, node);
    }

    fn visit_array_item_value(&mut self, node: &ArrayItemValue) {
        walk_array_item_value(self, node);
    }

    fn visit_array_item_referenced_value(&mut self, node: &ArrayItemReferencedValue) {
        walk_array_item_referenced_value(self, node);
    }

    fn visit_array_item_spread_value(&mut self, node: &ArrayItemSpreadValue) {
        walk_array_item_spread_value(self, node);
    }

    fn visit_array_item_key_value(&mut self, node: &ArrayItemKeyValue) {
        walk_array_item_key_value(self, node);
    }

    fn visit_array_item_referenced_key_value(&mut self, node: &ArrayItemReferencedKeyValue) {
        walk_array_item_referenced_key_value(self, node);
    }

    fn visit_list_entry(&mut self, node: &ListEntry) {
        walk_list_entry(self, node);
    }

    fn visit_list_entry_value(&mut self, node: &ListEntryValue) {
        walk_list_entry_value(self, node);
    }

    fn visit_list_entry_key_value(&mut self, node: &ListEntryKeyValue) {
        walk_list_entry_key_value(self, node);
    }

    fn visit_positional_argument(&mut self, node: &PositionalArgument) {
        walk_positional_argument(self, node);
    }

    fn visit_named_argument(&mut self, node: &NamedArgument) {
        walk_named_argument(self, node);
    }

    fn visit_argument(&mut self, node: &Argument) {
        walk_argument(self, node);
    }

    fn visit_argument_list(&mut self, node: &ArgumentList) {
        walk_argument_list(self, node);
    }

    fn visit_single_argument(&mut self, node: &SingleArgument) {
        walk_single_argument(self, node);
    }

    fn visit_argument_placeholder(&mut self, node: &ArgumentPlaceholder) {}

    fn visit_attribute(&mut self, node: &Attribute) {
        walk_attribute(self, node);
    }

    fn visit_attribute_group(&mut self, node: &AttributeGroup) {
        walk_attribute_group(self, node);
    }

    fn visit_class_body(&mut self, node: &ClassBody) {
        walk_class_body(self, node);
    }

    fn visit_class_statement(&mut self, node: &ClassStatement) {
        walk_class_statement(self, node);
    }

    fn visit_anonymous_class_body(&mut self, node: &AnonymousClassBody) {
        walk_anonymous_class_body(self, node);
    }

    fn visit_anonymous_class_expression(&mut self, node: &AnonymousClassExpression) {
        walk_anonymous_class_expression(self, node);
    }

    fn visit_class_extends(&mut self, node: &ClassExtends) {
        walk_class_extends(self, node);
    }

    fn visit_class_implements(&mut self, node: &ClassImplements) {
        walk_class_implements(self, node);
    }

    fn visit_classish_member(&mut self, node: &ClassishMember) {
        walk_classish_member(self, node);
    }

    fn visit_missing_classish_member(&mut self, node: &MissingClassishMember) {}

    fn visit_constant_entry(&mut self, node: &ConstantEntry) {
        walk_constant_entry(self, node);
    }

    fn visit_classish_constant_entry(&mut self, node: &ClassishConstantEntry) {
        walk_classish_constant_entry(self, node);
    }

    fn visit_constant_statement(&mut self, node: &ConstantStatement) {
        walk_constant_statement(self, node);
    }

    fn visit_classish_constant(&mut self, node: &ClassishConstant) {
        walk_classish_constant(self, node);
    }

    fn visit_if_statement(&mut self, node: &IfStatement) {
        walk_if_statement(self, node);
    }

    fn visit_if_statement_body(&mut self, node: &IfStatementBody) {
        walk_if_statement_body(self, node);
    }

    fn visit_if_statement_body_statement(&mut self, node: &IfStatementBodyStatement) {
        walk_if_statement_body_statement(self, node);
    }

    fn visit_if_statement_body_block(&mut self, node: &IfStatementBodyBlock) {
        walk_if_statement_body_block(self, node);
    }

    fn visit_if_statement_else_if(&mut self, node: &IfStatementElseIf) {
        walk_if_statement_else_if(self, node);
    }

    fn visit_if_statement_else(&mut self, node: &IfStatementElse) {
        walk_if_statement_else(self, node);
    }

    fn visit_if_statement_else_if_block(&mut self, node: &IfStatementElseIfBlock) {
        walk_if_statement_else_if_block(self, node);
    }

    fn visit_if_statement_else_block(&mut self, node: &IfStatementElseBlock) {
        walk_if_statement_else_block(self, node);
    }

    fn visit_data_type(&mut self, node: &DataType) {
        walk_data_type(self, node);
    }

    fn visit_declare_entry(&mut self, node: &DeclareEntry) {
        walk_declare_entry(self, node);
    }

    fn visit_declare_entry_group(&mut self, node: &DeclareEntryGroup) {
        walk_declare_entry_group(self, node);
    }

    fn visit_declare_body(&mut self, node: &DeclareBody) {
        walk_declare_body(self, node);
    }

    fn visit_declare_body_noop(&mut self, node: &DeclareBodyNoop) {}

    fn visit_declare_body_braced(&mut self, node: &DeclareBodyBraced) {
        walk_declare_body_braced(self, node);
    }

    fn visit_declare_body_expression(&mut self, node: &DeclareBodyExpression) {
        walk_declare_body_expression(self, node);
    }

    fn visit_declare_body_block(&mut self, node: &DeclareBodyBlock) {
        walk_declare_body_block(self, node);
    }

    fn visit_declare_statement(&mut self, node: &DeclareStatement) {
        walk_declare_statement(self, node);
    }

    fn visit_unit_enum_case(&mut self, node: &UnitEnumCase) {
        walk_unit_enum_case(self, node);
    }

    fn visit_unit_enum_member(&mut self, node: &UnitEnumMember) {
        walk_unit_enum_member(self, node);
    }

    fn visit_unit_enum_body(&mut self, node: &UnitEnumBody) {
        walk_unit_enum_body(self, node);
    }

    fn visit_unit_enum_statement(&mut self, node: &UnitEnumStatement) {
        walk_unit_enum_statement(self, node);
    }

    fn visit_backed_enum_case(&mut self, node: &BackedEnumCase) {
        walk_backed_enum_case(self, node);
    }

    fn visit_backed_enum_member(&mut self, node: &BackedEnumMember) {
        walk_backed_enum_member(self, node);
    }

    fn visit_backed_enum_body(&mut self, node: &BackedEnumBody) {
        walk_backed_enum_body(self, node);
    }

    fn visit_backed_enum_statement(&mut self, node: &BackedEnumStatement) {
        walk_backed_enum_statement(self, node);
    }

    fn visit_backed_enum_type(&mut self, node: &BackedEnumType) {
        walk_backed_enum_type(self, node);
    }

    fn visit_return_type(&mut self, node: &ReturnType) {
        walk_return_type(self, node);
    }

    fn visit_function_parameter(&mut self, node: &FunctionParameter) {
        walk_function_parameter(self, node);
    }

    fn visit_function_parameter_list(&mut self, node: &FunctionParameterList) {
        walk_function_parameter_list(self, node);
    }

    fn visit_function_body(&mut self, node: &FunctionBody) {
        walk_function_body(self, node);
    }

    fn visit_function_statement(&mut self, node: &FunctionStatement) {
        walk_function_statement(self, node);
    }

    fn visit_closure_use_variable(&mut self, node: &ClosureUseVariable) {
        walk_closure_use_variable(self, node);
    }

    fn visit_closure_use(&mut self, node: &ClosureUse) {
        walk_closure_use(self, node);
    }

    fn visit_closure_expression(&mut self, node: &ClosureExpression) {
        walk_closure_expression(self, node);
    }

    fn visit_arrow_function_expression(&mut self, node: &ArrowFunctionExpression) {
        walk_arrow_function_expression(self, node);
    }

    fn visit_constructor_parameter(&mut self, node: &ConstructorParameter) {
        walk_constructor_parameter(self, node);
    }

    fn visit_constructor_parameter_list(&mut self, node: &ConstructorParameterList) {
        walk_constructor_parameter_list(self, node);
    }

    fn visit_abstract_constructor(&mut self, node: &AbstractConstructor) {
        walk_abstract_constructor(self, node);
    }

    fn visit_concrete_constructor(&mut self, node: &ConcreteConstructor) {
        walk_concrete_constructor(self, node);
    }

    fn visit_abstract_method(&mut self, node: &AbstractMethod) {
        walk_abstract_method(self, node);
    }

    fn visit_concrete_method(&mut self, node: &ConcreteMethod) {
        walk_concrete_method(self, node);
    }

    fn visit_method_body(&mut self, node: &MethodBody) {
        walk_method_body(self, node);
    }

    fn visit_label_statement(&mut self, node: &LabelStatement) {
        walk_label_statement(self, node);
    }

    fn visit_goto_statement(&mut self, node: &GotoStatement) {
        walk_goto_statement(self, node);
    }

    fn visit_identifier(&mut self, node: &Identifier) {
        walk_identifier(self, node);
    }

    fn visit_simple_identifier(&mut self, node: &SimpleIdentifier) {}

    fn visit_dynamic_identifier(&mut self, node: &DynamicIdentifier) {
        walk_dynamic_identifier(self, node);
    }

    fn visit_interface_extends(&mut self, node: &InterfaceExtends) {
        walk_interface_extends(self, node);
    }

    fn visit_interface_body(&mut self, node: &InterfaceBody) {
        walk_interface_body(self, node);
    }

    fn visit_interface_statement(&mut self, node: &InterfaceStatement) {
        walk_interface_statement(self, node);
    }

    fn visit_literal(&mut self, node: &Literal) {
        walk_literal(self, node);
    }

    fn visit_literal_kind(&mut self, node: &LiteralKind) {
        walk_literal_kind(self, node);
    }

    fn visit_foreach_statement(&mut self, node: &ForeachStatement) {
        walk_foreach_statement(self, node);
    }

    fn visit_foreach_statement_iterator(&mut self, node: &ForeachStatementIterator) {
        walk_foreach_statement_iterator(self, node);
    }

    fn visit_foreach_statement_iterator_value(&mut self, node: &ForeachStatementIteratorValue) {
        walk_foreach_statement_iterator_value(self, node);
    }

    fn visit_foreach_statement_iterator_key_and_value(
        &mut self,
        node: &ForeachStatementIteratorKeyAndValue,
    ) {
        walk_foreach_statement_iterator_key_and_value(self, node);
    }

    fn visit_foreach_statement_body(&mut self, node: &ForeachStatementBody) {
        walk_foreach_statement_body(self, node);
    }

    fn visit_foreach_statement_body_statement(&mut self, node: &ForeachStatementBodyStatement) {
        walk_foreach_statement_body_statement(self, node);
    }

    fn visit_foreach_statement_body_block(&mut self, node: &ForeachStatementBodyBlock) {
        walk_foreach_statement_body_block(self, node);
    }

    fn visit_for_statement(&mut self, node: &ForStatement) {
        walk_for_statement(self, node);
    }

    fn visit_for_statement_iterator(&mut self, node: &ForStatementIterator) {
        walk_for_statement_iterator(self, node);
    }

    fn visit_for_statement_body(&mut self, node: &ForStatementBody) {
        walk_for_statement_body(self, node);
    }

    fn visit_for_statement_body_statement(&mut self, node: &ForStatementBodyStatement) {
        walk_for_statement_body_statement(self, node);
    }

    fn visit_for_statement_body_block(&mut self, node: &ForStatementBodyBlock) {
        walk_for_statement_body_block(self, node);
    }

    fn visit_do_while_statement(&mut self, node: &DoWhileStatement) {
        walk_do_while_statement(self, node);
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) {
        walk_while_statement(self, node);
    }

    fn visit_while_statement_body(&mut self, node: &WhileStatementBody) {
        walk_while_statement_body(self, node);
    }

    fn visit_while_statement_body_statement(&mut self, node: &WhileStatementBodyStatement) {
        walk_while_statement_body_statement(self, node);
    }

    fn visit_while_statement_body_block(&mut self, node: &WhileStatementBodyBlock) {
        walk_while_statement_body_block(self, node);
    }

    fn visit_level(&mut self, node: &Level) {
        walk_level(self, node);
    }

    fn visit_literal_level(&mut self, node: &LiteralLevel) {
        walk_literal_level(self, node);
    }

    fn visit_parenthesized_level(&mut self, node: &ParenthesizedLevel) {}

    fn visit_break_statement(&mut self, node: &BreakStatement) {
        walk_break_statement(self, node);
    }

    fn visit_continue_statement(&mut self, node: &ContinueStatement) {
        walk_continue_statement(self, node);
    }

    fn visit_visibility_modifier(&mut self, node: &VisibilityModifier) {}

    fn visit_promoted_property_modifier(&mut self, node: &PromotedPropertyModifier) {}

    fn visit_promoted_property_modifier_group(&mut self, node: &PromotedPropertyModifierGroup) {
        walk_promoted_property_modifier_group(self, node);
    }

    fn visit_property_modifier(&mut self, node: &PropertyModifier) {}

    fn visit_property_modifier_group(&mut self, node: &PropertyModifierGroup) {
        walk_property_modifier_group(self, node);
    }

    fn visit_method_modifier(&mut self, node: &MethodModifier) {}

    fn visit_method_modifier_group(&mut self, node: &MethodModifierGroup) {
        walk_method_modifier_group(self, node);
    }

    fn visit_class_modifier(&mut self, node: &ClassModifier) {}

    fn visit_class_modifier_group(&mut self, node: &ClassModifierGroup) {
        walk_class_modifier_group(self, node);
    }

    fn visit_constant_modifier(&mut self, node: &ConstantModifier) {}

    fn visit_constant_modifier_group(&mut self, node: &ConstantModifierGroup) {
        walk_constant_modifier_group(self, node);
    }

    fn visit_unbraced_namespace(&mut self, node: &UnbracedNamespace) {
        walk_unbraced_namespace(self, node);
    }

    fn visit_braced_namespace(&mut self, node: &BracedNamespace) {
        walk_braced_namespace(self, node);
    }

    fn visit_braced_namespace_body(&mut self, node: &BracedNamespaceBody) {
        walk_braced_namespace_body(self, node);
    }

    fn visit_namespace_statement(&mut self, node: &NamespaceStatement) {
        walk_namespace_statement(self, node);
    }

    fn visit_arithmetic_operation_expression(&mut self, node: &ArithmeticOperationExpression) {
        walk_arithmetic_operation_expression(self, node);
    }

    fn visit_arithmetic_operation_kind(&mut self, node: &ArithmeticOperationKind) {
        walk_arithmetic_operation_kind(self, node);
    }

    fn visit_assignment_operation_expression(&mut self, node: &AssignmentOperationExpression) {
        walk_assignment_operation_expression(self, node);
    }

    fn visit_assignment_operation_kind(&mut self, node: &AssignmentOperationKind) {
        walk_assignment_operation_kind(self, node);
    }

    fn visit_bitwise_operation_expression(&mut self, node: &BitwiseOperationExpression) {
        walk_bitwise_operation_expression(self, node);
    }

    fn visit_bitwise_operation_kind(&mut self, node: &BitwiseOperationKind) {
        walk_bitwise_operation_kind(self, node);
    }

    fn visit_comparison_operation_expression(&mut self, node: &ComparisonOperationExpression) {
        walk_comparison_operation_expression(self, node);
    }

    fn visit_comparison_operation_kind(&mut self, node: &ComparisonOperationKind) {
        walk_comparison_operation_kind(self, node);
    }

    fn visit_logical_operation_expression(&mut self, node: &LogicalOperationExpression) {
        walk_logical_operation_expression(self, node);
    }

    fn visit_logical_operation_kind(&mut self, node: &LogicalOperationKind) {
        walk_logical_operation_kind(self, node);
    }

    fn visit_name(&mut self, node: &Name) {
        walk_name(self, node);
    }

    fn visit_name_kind(&mut self, node: &NameKind) {
        walk_name_kind(self, node);
    }

    fn visit_special_name(&mut self, node: &SpecialName) {
        walk_special_name(self, node);
    }

    fn visit_special_name_kind(&mut self, node: &SpecialNameKind) {}

    fn visit_unresolved_name(&mut self, node: &UnresolvedName) {}

    fn visit_resolved_name(&mut self, node: &ResolvedName) {}

    fn visit_property(&mut self, node: &Property) {
        walk_property(self, node);
    }

    fn visit_variable_property(&mut self, node: &VariableProperty) {
        walk_variable_property(self, node);
    }

    fn visit_property_entry(&mut self, node: &PropertyEntry) {
        walk_property_entry(self, node);
    }

    fn visit_property_entry_kind(&mut self, node: &PropertyEntryKind) {
        walk_property_entry_kind(self, node);
    }

    fn visit_uninitialized_property_entry(&mut self, node: &UninitializedPropertyEntry) {
        walk_uninitialized_property_entry(self, node);
    }

    fn visit_initialized_property_entry(&mut self, node: &InitializedPropertyEntry) {
        walk_initialized_property_entry(self, node);
    }

    fn visit_trait_body(&mut self, node: &TraitBody) {
        walk_trait_body(self, node);
    }

    fn visit_trait_statement(&mut self, node: &TraitStatement) {
        walk_trait_statement(self, node);
    }

    fn visit_trait_usage(&mut self, node: &TraitUsage) {
        walk_trait_usage(self, node);
    }

    fn visit_trait_usage_adaptation(&mut self, node: &TraitUsageAdaptation) {
        walk_trait_usage_adaptation(self, node);
    }

    fn visit_trait_usage_adaptation_kind(&mut self, node: &TraitUsageAdaptationKind) {
        walk_trait_usage_adaptation_kind(self, node);
    }

    fn visit_trait_usage_adaptation_alias(&mut self, node: &TraitUsageAdaptationAlias) {
        walk_trait_usage_adaptation_alias(self, node);
    }

    fn visit_trait_usage_adaptation_visibility(&mut self, node: &TraitUsageAdaptationVisibility) {
        walk_trait_usage_adaptation_visibility(self, node);
    }

    fn visit_trait_usage_adaptation_precedence(&mut self, node: &TraitUsageAdaptationPrecedence) {
        walk_trait_usage_adaptation_precedence(self, node);
    }

    fn visit_catch_type(&mut self, node: &CatchType) {
        walk_catch_type(self, node);
    }

    fn visit_catch_type_kind(&mut self, node: &CatchTypeKind) {
        walk_catch_type_kind(self, node);
    }

    fn visit_catch_type_kind_identifier(&mut self, node: &CatchTypeKindIdentifier) {
        walk_catch_type_kind_identifier(self, node);
    }

    fn visit_catch_type_kind_union(&mut self, node: &CatchTypeKindUnion) {
        walk_catch_type_kind_union(self, node);
    }

    fn visit_try_statement(&mut self, node: &TryStatement) {
        walk_try_statement(self, node);
    }

    fn visit_catch_block(&mut self, node: &CatchBlock) {
        walk_catch_block(self, node);
    }

    fn visit_finally_block(&mut self, node: &FinallyBlock) {
        walk_finally_block(self, node);
    }

    fn visit_variable(&mut self, node: &Variable) {
        walk_variable(self, node);
    }

    fn visit_simple_variable(&mut self, node: &SimpleVariable) {}

    fn visit_variable_variable(&mut self, node: &VariableVariable) {
        walk_variable_variable(self, node);
    }

    fn visit_braced_variable_variable(&mut self, node: &BracedVariableVariable) {
        walk_braced_variable_variable(self, node);
    }

    fn visit_ending(&mut self, node: &Ending) {}

    fn visit_static_statement(&mut self, node: &StaticStatement) {
        walk_static_statement(self, node);
    }

    fn visit_switch_statement(&mut self, node: &SwitchStatement) {
        walk_switch_statement(self, node);
    }

    fn visit_echo_statement(&mut self, node: &EchoStatement) {
        walk_echo_statement(self, node);
    }

    fn visit_return_statement(&mut self, node: &ReturnStatement) {
        walk_return_statement(self, node);
    }

    fn visit_use_statement(&mut self, node: &UseStatement) {
        walk_use_statement(self, node);
    }

    fn visit_group_use_statement(&mut self, node: &GroupUseStatement) {
        walk_group_use_statement(self, node);
    }

    fn visit_halt_compiler_statement(&mut self, node: &HaltCompilerStatement) {}

    fn visit_static_var(&mut self, node: &StaticVar) {
        walk_static_var(self, node);
    }

    fn visit_comment(&mut self, node: &Comment) {
        walk_comment(self, node);
    }

    fn visit_comment_format(&mut self, node: &CommentFormat) {
        walk_comment_format(self, node);
    }

    fn visit_comment_group(&mut self, node: &CommentGroup) {}
}
