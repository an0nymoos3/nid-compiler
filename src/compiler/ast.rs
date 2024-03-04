/**
 * Define some types that will be required for parsing.
 */
#[allow(dead_code)]
enum NodeType {
    Program,             // Represents an entire program
    FuncDecleration,     // Function decleration (eg. void main())
    VarDecleration,      // Variable decleration (eg. int x)
    AssignmentStatement, // Represents assignment (eg. x = 5)
    Expression,          // An expression can be binary, unary or literal
    ConditionStatement,  // If, else if, else
    LoopStatement,       // While loops (maybe for in the future)
    BlockStatement,      // A block within {} could be used to scope variables
    ReturnStatement,     // Return statements (eg. return x)
    FunctionCall,        // Function call (eg. my_func())
    UnaryExpression,     // Unary operators (eg. !condition)
    BinaryExpression,    // Binary expressions, like arithmetic
    Literal,             // String or numeric literal
    ArrayType,           // Arrays are cool
    PointerType,         // Pointers and refrences can be useful
    RefrenceType,        // For handling refrences
    StructDecleration,   // Structs are cool
    EnumDecleration,     // Enums could be good
    TypeSpecifier,       // Type specifier like int, string, float, etc
    ArrayAccess,         // Access in array could be good
    Eol,                 // To allow lines to span multiple editor lines
}
