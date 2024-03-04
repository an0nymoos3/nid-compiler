#[allow(dead_code)] // TODO: Remove later
enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

#[allow(dead_code)] // TODO: Remove later
struct BlockStatement {
    body: Vec<NodeType>,
}

struct ConditionStatemet {}

struct LoopStatement {
    condition: ConditionStatemet,
    body: BlockStatement,
}

struct ArrayStatement {
    size: i32,
    datatype: Value,
}

/**
 * Define some types that will be required for parsing.
 */
#[allow(dead_code)] // TODO: Remove later
enum NodeType {
    FuncDecleration(String),        // Function decleration (eg. void main())
    VarDecleration(String),         // Variable decleration (eg. int x)
    AssignmentStatement(String),    // Represents assignment (eg. x = 5)
    Expression,                     // An expression can be binary, unary or literal
    ConditionStatement,             // If, else if, else
    LoopStatement(LoopStatement),   // While loops (maybe for in the future)
    BlockStatement(BlockStatement), // A block within {} could be used to scope variables
    ReturnStatement,                // Return statements (eg. return x)
    FunctionCall(String),           // Function call (eg. my_func())
    UnaryExpression,                // Unary operators (eg. !condition)
    BinaryExpression,               // Binary expressions, like arithmetic
    Literal(Value),                 // String or numeric literal
    ArrayType(ArrayStatement),      // Arrays are cool
    PointerType(String),            // Pointers and refrences can be useful
    RefrenceType(String),           // For handling refrences
    StructDecleration(String),      // Structs are cool
    EnumDecleration(String),        // Enums could be good
    TypeSpecifier(String),          // Type specifier like int, string, float, etc
    ArrayAccess,                    // Access in array could be good
    Eol,                            // To allow lines to span multiple editor lines
}
