// todo: this should probably have a type field
pub trait ExprAST {
    fn print(&self);
}

// Expression class for referencing a variable
pub struct VariableExprAST {
    pub name: String,
}

impl ExprAST for VariableExprAST 
{
    fn print(&self) {
        println!("VariableExprAST: {}", self.name);
    }
}

// Expression class for a binary operator
#[allow(dead_code)]
pub struct BinaryExprAST {
    pub op: char,
    pub lhs: Box<dyn ExprAST>,
    pub rhs: Box<dyn ExprAST>,
}

impl ExprAST for BinaryExprAST {
    fn print(&self) {
        println!("BinaryExprAST: {} ", self.op);
    }
}

// Expression class for function calls
pub struct CallExprAST {
    pub function: String,
    pub args: Vec<Box<dyn ExprAST>>,
}

impl ExprAST for CallExprAST 
{
    fn print(&self) {
        println!("CallExprAST: {}", self.function);
        for arg in &self.args {
           arg.print(); 
        }
    }
}

// Represents the "prototype" for a function
// Captures its name, and its arguments names
pub struct PrototypeAST {
    pub name: String,
    pub args: Vec<String>,
}

impl ExprAST for PrototypeAST {
    // todo: improve formatting, it's late here. 
    fn print(&self) {
        println!("PrototypeAST: {}", self.name);
        for arg in &self.args {
            print!("{}", arg);
        }
    }
}


// Represents a function definition
#[allow(dead_code)]
pub struct FunctionAST {
    pub proto: Box<PrototypeAST>, // should this be heap allocated?
    pub body: Box<dyn ExprAST>,

}

impl ExprAST for FunctionAST {
    fn print(&self) {
        todo!() // too tired...
    }
}

#[allow(dead_code)]
pub struct CanonicalParser {
    pub expressions: Vec<Box<dyn ExprAST>>,
}

impl Default for CanonicalParser{
    fn default() -> Self {
        CanonicalParser { expressions: Vec::new() }
    }
}
