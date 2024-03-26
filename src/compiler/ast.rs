use std::fmt;

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
    pub value: String,
}

#[derive(Debug)]
pub struct Ast {
    pub body: Vec<NodeType>,
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
    Debug(Node), // Meant for debugging, when specific type is not required
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FuncDecleration(value) => write!(f, "{value}"),
            Self::VarDecleration(value) => write!(f, "{value}"),
            Self::AssignmentStatement(value) => write!(f, "{value}"),
            Self::BranchStatement(_) => write!(f, "BRANCH"),
            Self::LoopStatement(_) => write!(f, "LOOP"),
            Self::BlockStatement(_) => write!(f, "SCOPE"),
            Self::ReturnStatement => write!(f, "RETURN"),
            Self::FunctionCall(value) => write!(f, "{value}"),
            Self::UnaryExpression(_) => write!(f, "UNARY"),
            Self::BinaryExpression(_) => write!(f, "BINARY"),
            Self::Literal(_) => write!(f, "LITERAL"),
            Self::ArrayDecleration(_) => write!(f, "ARRAYDEC"),
            Self::PointerType(value) => write!(f, "*{value}"),
            Self::RefrenceType(value) => write!(f, "&{value}"),
            Self::StructDecleration(value) => write!(f, "struct {value}"),
            Self::EnumDecleration(value) => write!(f, "enum {value}"),
            Self::TypeSpecifier(value) => write!(f, "Type: {value}"),
            Self::ArrayAccess(value) => write!(f, "array: {value}"),
            Self::Debug(node) => write!(f, "{}", node.value),
            _ => write!(f, "UNKNOWN!"),
        }
    }
}
