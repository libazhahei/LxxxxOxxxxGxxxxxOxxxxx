use crate::transpiler::Translater;
use crate::{
    ast::{Executable, Statement},
    heap::Heap,
    tokens::ASTParser,
};
use std::panic::set_hook;
use unsvg::Image;

pub struct Program {
    statements: Vec<Statement>,
    heap: Heap,
}
#[allow(dead_code)]
impl Program {
    pub fn new(statements: Vec<Statement>, heap: Heap) -> Program {
        Program { heap, statements }
    }
    pub fn parse_logo(logo: &String, image: Image) -> Program {
        set_hook(Box::new(|panic_info| {
            println!("Panic: {:?}", panic_info.to_string());
            std::process::exit(1);
        }));
        let mut parser = ASTParser::new(logo);
        let statements = parser.parse_statement();
        Program::new(statements, Heap::new(image))
    }
    pub fn run(&mut self) {
        for statement in &self.statements {
            statement.execute(&mut self.heap);
        }
    }
    pub fn image(&self) -> &Image {
        self.heap.turtle().image()
    }
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
    pub fn heap(&self) -> &Heap {
        &self.heap
    }
    // /*
    pub fn to_python(&self) -> String {
        let mut buf = String::new();
        for statement in &self.statements {
            buf.push_str(&statement.to_python(0));
        }
        buf
    }
    // */
}
