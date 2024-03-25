use std::{collections::HashMap, rc::Rc};

use unsvg::Image;

use crate::{
    ast::Callee,
    commands::{Assignable, AssignableFactory, Callable, CallableFactory, Literal, Value},
    tokens::AssignmentType,
    turtle::Turtle,
};

pub struct Heap {
    variables: HashMap<String, Literal>,
    assignables: HashMap<AssignmentType, Rc<dyn Assignable>>,
    callables: HashMap<Callee, Rc<dyn Callable>>,
    turtle: Turtle,
}
impl Heap {
    pub fn new(image: Image) -> Heap {
        Heap {
            variables: HashMap::new(),
            turtle: Turtle::new_with_img(image),
            callables: HashMap::new(),
            assignables: HashMap::new(),
        }
    }
    pub fn mut_variables(&mut self) -> &mut HashMap<String, Literal> {
        &mut self.variables
    }

    pub fn variable_value(&self, name: &String) -> Option<Literal> {
        self.variables.get(name).cloned()
    }

    pub fn mut_turtle(&mut self) -> &mut Turtle {
        &mut self.turtle
    }
    pub fn turtle(&self) -> &Turtle {
        &self.turtle
    }

    pub fn get_callable(&mut self, callee: &Callee) -> &Rc<dyn Callable> {
        if !self.callables.contains_key(callee) {
            let built_in = CallableFactory::build(callee);
            if let Some(built_in) = built_in {
                self.callables.insert(callee.clone(), built_in);
            } else {
                panic!("Function {:?} is not found", callee);
            }
        };
        self.callables.get(callee).unwrap()
    }
    pub fn get_assignable(&mut self, assign_type: &AssignmentType) -> &Rc<dyn Assignable> {
        if !self.assignables.contains_key(assign_type) {
            let built_in = AssignableFactory::build(assign_type);
            if let Some(built_in) = built_in {
                self.assignables.insert(assign_type.clone(), built_in);
            } else {
                panic!("Function is not found");
            }
        };
        self.assignables.get(assign_type).unwrap()
    }
    pub fn execute_function(&mut self, callee: &Callee, args: &[Value]) -> Option<Literal> {
        let function: Rc<dyn Callable> = self.get_callable(callee).clone();
        function.call(args, self)
    }
    pub fn declare_variable(&mut self, assign_type: &AssignmentType) -> Rc<dyn Assignable> {
        self.get_assignable(assign_type).clone()
    }
    pub fn declare_procedure(&mut self, name: String, callable: Rc<dyn Callable>) {
        self.callables.insert(Callee::Procedure(name), callable);
    }
}
