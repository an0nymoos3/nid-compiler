#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Void,
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeType,
    pub value: String,
}

#[derive(Debug)]
pub struct Ast {
    pub body: Vec<Node>,
}

#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<NodeType>,
}

#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: String,
    pub operand: String,
}

#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub struct BinaryExpression {
    pub operator: String,
    pub l_operand: String,
    pub r_operand: String,
}

#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub struct ConditionStatemet {}

#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub struct BranchStatement {
    pub condition: ConditionStatemet,
    pub body: BlockStatement,
}

#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub struct ArrayStatement {
    pub size: i32,
    pub datatype: Value,
}

/**
 * Define some types that will be required for parsing.
 */
#[allow(dead_code)] // TODO: Remove later
#[derive(Debug)]
pub enum NodeType {
    FuncDecleration(String),            // Function decleration (eg. void main())
    VarDecleration(String),             // Variable decleration (eg. int x)
    AssignmentStatement(String),        // Represents assignment (eg. x = 5)
    BranchStatement(BranchStatement),   // If, else if, else
    LoopStatement(BranchStatement),     // While loops (maybe for in the future)
    BlockStatement(BlockStatement),     // A block within {} could be used to scope variables
    ReturnStatement,                    // Return statements (eg. return x)
    FunctionCall(String),               // Function call (eg. my_func())
    UnaryExpression(UnaryExpression),   // Unary operators (eg. !condition)
    BinaryExpression(BinaryExpression), // Binary expressions, like arithmetic
    Literal(Value),                     // String or numeric literal
    ArrayDecleration(ArrayStatement),   // Arrays are cool
    PointerType(String),                // Pointers and refrences can be useful
    RefrenceType(String),               // For handling refrences
    StructDecleration(String),          // Structs are cool
    EnumDecleration(String),            // Enums could be good
    TypeSpecifier(String),              // Type specifier like int, string, float, etc
    ArrayAccess(i32),                   // Access in array could be good
    Eol,                                // To allow lines to span multiple editor lines
    //
    Debug, // Meant for debugging, when specific type is not required
}
