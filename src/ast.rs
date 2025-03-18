/// Represents the top-level structure of a program, which is a sequence of elements.
/// This is the root node of the AST, containing all top-level constructs.
///
/// The parser should iterate over the token stream, identifying comments, items,
/// module declarations, and use statements, and collect them into the `elements` vector.
pub struct Program {
    /// A vector of top-level elements in the order they appear in the source code.
    pub elements: Vec<TopLevelElement>,
}

/// Enumerates the possible elements that can appear at the top level of a program.
/// These are the building blocks of the program's structure.
///
/// The parser should match tokens to determine which variant to construct:
/// - `#` or `#*` for comments
/// - `mod` for module declarations
/// - `use` for use statements
/// - `pub`, `proto`, `struct`, `enum`, `fn`, or `const` for items
pub enum TopLevelElement {
    Comment(Comment),
    Item(Item),
    ModDeclaration(ModDeclaration),
    UseStatement(UseStatement),
}

/// Represents a comment in the source code, either single-line or multi-line.
///
/// The parser should:
/// - For `#`, collect all characters until a newline into `SingleLine`.
/// - For `#*`, collect all characters until `*#` into `MultiLine`.
pub enum Comment {
    /// A single-line comment starting with `#`, e.g., `# This is a comment`.
    SingleLine(String),
    /// A multi-line comment enclosed between `#*` and `*#`, e.g., `#* Multi-line *#`.
    MultiLine(String),
}

/// Represents a module declaration, e.g., `mod some_module;`.
///
/// The parser should:
/// - Expect `mod` followed by an identifier and a semicolon.
/// - Store the identifier in `name`.
pub struct ModDeclaration {
    /// The name of the module.
    pub name: String,
}

/// Represents a use statement for importing paths, e.g., `use some_module::say_hello;`.
///
/// The parser should:
/// - Expect `use` followed by a path and a semicolon.
/// - Delegate path parsing to `Path`.
pub struct UseStatement {
    /// The path being imported.
    pub path: Path,
}

/// Represents a hierarchical path, e.g., `some_module::say_hello`.
///
/// The parser should:
/// - Collect a sequence of identifiers separated by `::`.
/// - Store each identifier in the `segments` vector in order.
pub struct Path {
    /// A vector of path segments, e.g., `["some_module", "say_hello"]`.
    pub segments: Vec<String>,
}

/// Enumerates the possible items (definitions) that can appear in a program.
///
/// The parser should:
/// - Look for keywords like `proto`, `struct`, `enum`, `fn`, or `const`, optionally preceded by `pub`.
/// - Construct the appropriate variant based on the keyword.
pub enum Item {
    Protocol(ProtocolDefinition),
    Struct(StructDefinition),
    Enum(EnumDefinition),
    Function(FunctionDefinition),
    Constant(ConstDefinition),
}

/// Represents a protocol (interface) definition, e.g., `pub proto MyProto<T>: OtherProto { ... }`.
///
/// The parser should:
/// - Check for `pub` (optional), then `proto`, an identifier, optional generics, optional inheritance,
///   and a block of methods enclosed in `{}`.
/// - Populate fields accordingly, parsing methods into `ProtocolMethod`.
pub struct ProtocolDefinition {
    /// Whether the protocol is public (`pub` present).
    pub is_public: bool,
    /// The name of the protocol.
    pub name: String,
    /// Optional generic parameters, e.g., `<T>`.
    pub generics: Option<GenericParams>,
    /// Optional list of protocols this protocol inherits from, e.g., `: OtherProto`.
    pub super_protocols: Option<Vec<ProtocolRef>>,
    /// The methods defined in the protocol.
    pub methods: Vec<ProtocolMethod>,
}

/// Represents a method signature within a protocol.
///
/// The parser should:
/// - Reuse the `FunctionDefinition` parsing logic, as protocol methods are function signatures.
/// - Wrap the result in this struct.
pub struct ProtocolMethod {
    /// The function definition representing the method.
    pub func: FunctionDefinition,
}

/// Represents a struct definition, e.g., `pub struct MyStruct: MyProto { ... }`.
///
/// The parser should:
/// - Check for `pub` (optional), then `struct`, an identifier, optional protocol conformance,
///   and a block of members enclosed in `{}`.
/// - Parse the block contents into `StructMember` variants.
pub struct StructDefinition {
    /// Whether the struct is public (`pub` present).
    pub is_public: bool,
    /// The name of the struct.
    pub name: String,
    /// Optional list of protocols this struct conforms to, e.g., `: MyProto`.
    pub protocols: Option<Vec<ProtocolRef>>,
    /// The members (fields, methods, or comments) within the struct.
    pub members: Vec<StructMember>,
}

/// Enumerates the possible members within a struct definition.
///
/// The parser should:
/// - Inside a struct’s `{}` block, identify comments, fields (`pub identifier: type;`),
///   or function definitions, and construct the appropriate variant.
pub enum StructMember {
    Comment(Comment),
    Field(StructField),
    Method(FunctionDefinition),
}

/// Represents a field within a struct, e.g., `pub x: int;`.
///
/// The parser should:
/// - Expect `pub`, an identifier, `:`, a type, and `;`.
/// - Store the identifier in `name` and parse the type into `ty`.
pub struct StructField {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: Type,
}

/// Represents an enum definition, e.g., `pub enum MyEnum<T> { ... }`.
///
/// The parser should:
/// - Check for `pub` (optional), then `enum`, an identifier, optional generics,
///   and a block of members enclosed in `{}`.
/// - Parse the block contents into `EnumMember` variants.
pub struct EnumDefinition {
    /// Whether the enum is public (`pub` present).
    pub is_public: bool,
    /// The name of the enum.
    pub name: String,
    /// Optional generic parameters, e.g., `<T>`.
    pub generics: Option<GenericParams>,
    /// The members (variants, methods, or comments) within the enum.
    pub members: Vec<EnumMember>,
}

/// Enumerates the possible members within an enum definition.
///
/// The parser should:
/// - Inside an enum’s `{}` block, identify comments, variants (e.g., `Variant;` or `Variant(Type);`),
///   or function definitions, and construct the appropriate variant.
pub enum EnumMember {
    Comment(Comment),
    Variant(EnumVariant),
    Method(FunctionDefinition),
}

/// Represents an enum variant, e.g., `Variant` or `Variant(int)` or `Variant { x: int }`.
///
/// The parser should:
/// - Expect an identifier followed by an optional payload (tuple or struct-like) and a semicolon.
/// - Parse the payload into `EnumVariantPayload` if present.
pub struct EnumVariant {
    /// The name of the variant.
    pub name: String,
    /// Optional payload (tuple or struct-like) associated with the variant.
    pub payload: Option<EnumVariantPayload>,
}

/// Enumerates the possible payloads for an enum variant.
///
/// The parser should:
/// - After the variant name, check for `(` (tuple) or `{` (struct-like).
/// - For `(type)`, parse the type into `Tuple`.
/// - For `{ fields }`, parse each field into `VariantField` and collect them into `Struct`.
pub enum EnumVariantPayload {
    /// A tuple-style payload with a single type, e.g., `(int)` in `Variant(int)`.
    Tuple(Type),
    /// A struct-style payload with named fields, e.g., `{ x: int }` in `Variant { x: int }`.
    Struct(Vec<VariantField>),
}

/// Represents a field within a struct-like enum variant, e.g., `x: int`.
///
/// The parser should:
/// - Within a variant’s `{}` block, expect an identifier, `:`, a type, and `;`.
/// - Store the identifier in `name` and parse the type into `ty`.
pub struct VariantField {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: Type,
}

/// Represents a function definition, e.g., `pub fn my_func<T>(self, x: int) -> bool { ... }`.
///
/// The parser should:
/// - Check for `pub` (optional), then `fn`, an identifier, optional generics, parameters in `()`,
///   an optional return type after `->`, and a block.
/// - Parse parameters into `FunctionParams` and the block into `Block`.
pub struct FunctionDefinition {
    /// Whether the function is public (`pub` present).
    pub is_public: bool,
    /// The name of the function.
    pub name: String,
    /// Optional generic parameters, e.g., `<T>`.
    pub generics: Option<GenericParams>,
    /// The function’s parameters, either `self` or a list of named parameters.
    pub params: FunctionParams,
    /// The optional return type, e.g., `-> int`.
    pub return_type: Option<Type>,
    /// The body of the function as a block.
    pub body: Block,
}

/// Enumerates the possible parameter forms for a function.
///
/// The parser should:
/// - Within `()`, check for `self` or `mut self` (for methods), or a comma-separated list of parameters.
/// - Construct `SelfOnly` for `self` cases, or `Params` for regular parameters.
pub enum FunctionParams {
    /// Represents a method with only a `self` parameter, optionally mutable.
    SelfOnly { mutable: bool },
    /// Represents a list of named parameters.
    Params(Vec<Parameter>),
}

/// Represents a single function parameter, e.g., `x: int`.
///
/// The parser should:
/// - Expect an identifier, `:`, and a type.
/// - Store the identifier in `name` and parse the type into `ty`.
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub ty: Type,
}

/// Represents a constant definition, e.g., `pub const MY_CONST: int = 42;`.
///
/// The parser should:
/// - Check for `pub` (optional), then `const`, an identifier, `:`, a type, `=`, an expression, and `;`.
/// - Parse the type into `ty` and the expression into `value`.
pub struct ConstDefinition {
    /// Whether the constant is public (`pub` present).
    pub is_public: bool,
    /// The name of the constant.
    pub name: String,
    /// The type of the constant.
    pub ty: Type,
    /// The value assigned to the constant.
    pub value: Expression,
}

/// Represents a reference to a protocol, e.g., `MyProto` or `MyProto<int>`.
///
/// The parser should:
/// - Expect an identifier, optionally followed by `<type>`.
/// - Parse the type into `generic_arg` if present.
pub struct ProtocolRef {
    /// The name of the protocol.
    pub name: String,
    /// Optional generic argument, e.g., `<int>` in `MyProto<int>`.
    pub generic_arg: Option<Type>,
}

/// Represents a list of generic parameters, e.g., `<T, U: Proto>`.
///
/// The parser should:
/// - Expect `<`, a comma-separated list of generic parameters, and `>`.
/// - Parse each parameter into `GenericParam`.
pub struct GenericParams {
    /// The list of generic parameters.
    pub params: Vec<GenericParam>,
}

/// Represents a single generic parameter, e.g., `T` or `T: Proto` or `T = int`.
///
/// The parser should:
/// - Expect an identifier, optionally followed by `: constraints` and/or `= type`.
/// - Parse constraints into `ProtocolRef` and the default type into `Type`.
pub struct GenericParam {
    /// The name of the generic parameter.
    pub name: String,
    /// Optional constraints (protocols the parameter must conform to).
    pub constraints: Option<Vec<ProtocolRef>>,
    /// Optional default type for the parameter.
    pub default_type: Option<Type>,
}

/// Enumerates the possible types in the language.
///
/// The parser should:
/// - Match basic types (`int`, `float`, etc.), identifiers, generic types (`Name<Type>`),
///   or array types (`[Type, Type]`) and construct the appropriate variant.
pub enum Type {
    /// A basic built-in type, e.g., `int`, `float`, `bool`, `char`, `str`.
    Basic(String),
    /// A user-defined type (an identifier), e.g., `MyStruct`.
    Named(String),
    /// A generic type, e.g., `List<int>`.
    Generic { name: String, arg: Box<Type> },
    /// An array or tuple type, e.g., `[int, bool]`.
    Array(Vec<Type>),
}

/// Represents a block of code, e.g., `{ stmt1; stmt2; expr }`.
///
/// The parser should:
/// - Expect `{`, a sequence of statements, an optional final expression, and `}`.
/// - Parse statements into `Statement` and the final expression into `Expression`.
pub struct Block {
    /// The list of statements within the block.
    pub statements: Vec<Statement>,
    /// The optional final expression, whose value is returned by the block.
    pub final_expr: Option<Box<Expression>>,
}

/// Enumerates the possible statements within a block.
///
/// The parser should:
/// - Identify `let` for variable definitions, `break`, `continue`, or standalone expressions
///   followed by `;`, and construct the appropriate variant.
pub enum Statement {
    Variable(VariableDefinition),
    Expression(Expression),
    Break(Option<Box<Expression>>),
    Continue,
}

/// Represents a variable definition, e.g., `let mut x: int = 42;`.
///
/// The parser should:
/// - Expect `let`, optionally `mut`, an identifier, optionally `: type`, `=`, an expression, and `;`.
/// - Parse the type into `ty` and the expression into `value`.
pub struct VariableDefinition {
    /// Whether the variable is mutable (`mut` present).
    pub is_mutable: bool,
    /// The name of the variable.
    pub name: String,
    /// The optional type annotation.
    pub ty: Option<Type>,
    /// The initial value of the variable.
    pub value: Expression,
}

/// Enumerates the possible expressions in the language.
///
/// The parser should:
/// - Recursively parse expressions based on operator precedence and syntax rules,
///   constructing the appropriate variant for literals, identifiers, operations, control flow, etc.
pub enum Expression {
    /// A literal value, e.g., `42`, `"hello"`.
    Literal(Literal),
    /// An identifier, e.g., `x`.
    Identifier(String),
    /// A binary operation, e.g., `x + y`.
    Binary {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// A unary operation, e.g., `-x`.
    Unary {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    /// An if expression, e.g., `if x { ... } else { ... }`.
    If {
        cond: Box<Expression>,
        then: Block,
        else_: Option<ElseClause>,
    },
    /// An unless expression, e.g., `unless x { ... } else { ... }`.
    Unless {
        cond: Box<Expression>,
        then: Block,
        else_: Option<Block>,
    },
    /// A block expression, e.g., `{ x + y }`.
    Block(Block),
    /// A function call, e.g., `foo(x, y)`.
    FunctionCall { name: String, args: Vec<Expression> },
    /// An infinite loop, e.g., `loop { ... }`.
    Loop(Block),
    /// A for loop, e.g., `for x in y { ... }`.
    For {
        var: String,
        iter: Box<Expression>,
        body: Block,
    },
    /// A while loop, e.g., `while x { ... }`.
    While { cond: Box<Expression>, body: Block },
    /// A range expression, e.g., `1..5` or `1..=5`.
    Range {
        start: Box<Expression>,
        inclusive: bool,
        end: Box<Expression>,
    },
    /// A match expression, e.g., `match x { ... }`.
    Match {
        expr: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    /// A struct literal, e.g., `MyStruct { x: 1 }`.
    StructLiteral {
        name: String,
        fields: Vec<FieldInit>,
    },
    /// An enum literal, e.g., `MyEnum::Variant(1)` or `MyEnum::Variant { x: 1 }`.
    EnumLiteral {
        enum_name: String,
        variant: String,
        payload: Option<EnumLiteralPayload>,
    },
    /// A tuple literal, e.g., `[1, 2, 3]`.
    Tuple(Vec<Expression>),
    /// A field access, e.g., `x.field`.
    FieldAccess {
        expr: Box<Expression>,
        field: String,
    },
    /// A method call, e.g., `x.method(y)`.
    MethodCall {
        receiver: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    /// A closure, e.g., `|x| x + 1` or `|x: int| -> int { x + 1 }`.
    Closure {
        params: Vec<ClosureParam>,
        return_type: Option<Type>,
        body: ClosureBody,
    },
}

/// Enumerates the possible literal values in the language.
///
/// The parser should:
/// - Match literal tokens (numbers, strings, etc.) and construct the appropriate variant.
/// - For strings, handle interpolation by parsing `#{}`
pub enum Literal {
    /// An integer literal, stored as a string to preserve format, e.g., "-42".
    Integer(String),
    /// A float literal, stored as a string, e.g., "3.14".
    Float(String),
    /// A boolean literal, e.g., `true` or `false`.
    Boolean(bool),
    /// A character literal, e.g., `'a'`.
    Char(char),
    /// A string literal, possibly with interpolated expressions, e.g., `"hello #{x}"`.
    String(Vec<StringContent>),
}

/// Represents the content of a string literal, either plain text or an interpolated expression.
///
/// The parser should:
/// - Split a string into segments, parsing `#{} as `Interpolated` and other characters as `Text`.
pub enum StringContent {
    /// Plain text within a string.
    Text(String),
    /// An interpolated expression, e.g., `#{x + 1}`.
    Interpolated(Box<Expression>),
}

/// Enumerates the possible binary operators.
///
/// The parser should:
/// - Match operator tokens (`+`, `-`, etc.) and map them to the corresponding variant.
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Or,
    And,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

/// Enumerates the possible unary operators.
///
/// The parser should:
/// - Match operator tokens (`-`, `!`, `~`) and map them to the corresponding variant.
pub enum UnaryOperator {
    Neg,
    Not,
    BitNot,
}

/// Enumerates the possible else clauses in an if expression.
///
/// The parser should:
/// - After `else`, check for `{` (block) or `if` (nested if) and construct accordingly.
pub enum ElseClause {
    Block(Block),
    If(Box<Expression>),
}

/// Represents a single arm in a match expression, e.g., `x if cond -> expr`.
///
/// The parser should:
/// - Expect a pattern, optional `if` guard, `->`, and an expression or block, followed by `,`.
pub struct MatchArm {
    /// The pattern to match against.
    pub pattern: Pattern,
    /// Optional guard condition.
    pub guard: Option<Box<Expression>>,
    /// The result of the arm (expression or block).
    pub result: MatchResult,
}

/// Enumerates the possible results of a match arm.
///
/// The parser should:
/// - After `->`, check for `{` (block) or an expression and construct accordingly.
pub enum MatchResult {
    Expression(Box<Expression>),
    Block(Block),
}

/// Enumerates the possible patterns in a match expression.
///
/// The parser should:
/// - Parse literals, identifiers, `_`, ranges, or-patterns, enum patterns, or tuple patterns.
pub enum Pattern {
    /// A literal pattern, e.g., `42`.
    Literal(Literal),
    /// An identifier pattern, e.g., `x`.
    Identifier(String),
    /// A wildcard pattern, `_`.
    Wildcard,
    /// A range pattern, e.g., `1..5` (exclusive) or `1..=5` (inclusive).
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        inclusive: bool, // True for `..=`, false for `..`
    },
    /// An or-pattern, e.g., `x | y`.
    Or(Box<Pattern>, Box<Pattern>),
    /// An enum pattern, e.g., `Variant(x)` or `Variant { x: y }`.
    Enum {
        name: String,
        payload: Option<EnumPatternPayload>,
    },
    /// A tuple pattern, e.g., `[x, y]`.
    Tuple(Vec<Pattern>),
}

/// Enumerates the possible payloads in an enum pattern.
///
/// The parser should:
/// - For `(identifier)`, store the binding in `Tuple`.
/// - For `{ fields }`, parse into `Struct`.
pub enum EnumPatternPayload {
    /// A tuple-style payload with a binding, e.g., `(x)`.
    Tuple(String),
    /// A struct-style payload with named fields, e.g., `{ x: y }`.
    Struct(Vec<PatternField>),
}

/// Represents a field in a struct-like enum pattern, e.g., `x: y`.
///
/// The parser should:
/// - Expect an identifier, `:`, and a pattern.
pub struct PatternField {
    /// The name of the field.
    pub name: String,
    /// The pattern for the field’s value.
    pub pattern: Pattern,
}

/// Represents a field initialization in a struct literal, e.g., `x: 1`.
///
/// The parser should:
/// - Expect an identifier, `:`, and an expression.
pub struct FieldInit {
    /// The name of the field.
    pub name: String,
    /// The value assigned to the field.
    pub value: Expression,
}

/// Enumerates the possible payloads in an enum literal.
///
/// The parser should:
/// - For `(expr)`, parse into `Tuple`.
/// - For `{ fields }`, parse into `Struct`.
pub enum EnumLiteralPayload {
    /// A tuple-style payload, e.g., `(1)`.
    Tuple(Box<Expression>),
    /// A struct-style payload, e.g., `{ x: 1 }`.
    Struct(Vec<FieldInit>),
}

/// Represents a parameter in a closure, e.g., `x` or `x: int`.
///
/// The parser should:
/// - Expect an identifier, optionally followed by `: type`.
pub struct ClosureParam {
    /// The name of the parameter.
    pub name: String,
    /// The optional type annotation.
    pub ty: Option<Type>,
}

/// Enumerates the possible bodies of a closure.
///
/// The parser should:
/// - After `|params|`, check for a single expression or `{}` with optional return type.
pub enum ClosureBody {
    /// A single expression body, e.g., `|x| x + 1`.
    Expression(Box<Expression>),
    /// A block body with optional return type, e.g., `|x| -> int { x + 1 }`.
    Block {
        return_type: Option<Type>,
        block: Block,
    },
}
