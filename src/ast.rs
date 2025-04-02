pub trait ExprAST {
    fn print(&self);
}

/**
 * Expression class for integer literals
 */
pub struct IntegerLiteralExprAST {
    pub value: i32,
}

impl ExprAST for IntegerLiteralExprAST {
    fn print(&self) {
        println!("IntegerLiteralExprAST: {}", self.value);
    }
}

/**
 * Expression class for a float literals
 */
pub struct FloatLiteralExprAST {
    pub value: f32,
}

impl ExprAST for FloatLiteralExprAST {
    fn print(&self) {
        println!("FloatLiteralExprAST: {}", self.value);
    }
}

/**
 * Expression class for referencing a variable
 */
pub struct VariableExprAST {
    pub name: String,
}

impl ExprAST for VariableExprAST 
{
    fn print(&self) {
        println!("VariableExprAST: {}", self.name);
    }
}

/**
 * Expression class for a binary operator
 */
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

/**
 * Expression class for function calls
 */
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

/**
 * Represents the "prototype" for a function
 */
pub struct PrototypeAST {
    pub name: String,
    pub args: Vec<String>,
}

impl ExprAST for PrototypeAST {
    fn print(&self) {
        println!("PrototypeAST: {}", self.name);
        for arg in &self.args {
            println!("-- arg: {}", arg);
        }
    }
}

/**
 * Represents a function definition
 */
#[allow(dead_code)]
pub struct FunctionAST {
    pub proto: Box<PrototypeAST>,
    pub body: Box<dyn ExprAST>,

}

impl ExprAST for FunctionAST {
    fn print(&self) {
        todo!() 
    }
}