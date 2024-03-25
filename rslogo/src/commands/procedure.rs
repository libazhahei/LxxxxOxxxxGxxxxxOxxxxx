use std::collections::HashMap;

use crate::{
    ast::{Executable, Statement},
    heap::Heap,
};

use super::{Callable, Literal, Procedure};

impl Callable for Procedure {
    fn call(&self, args: &[super::Value], heap: &mut Heap) -> Option<super::Literal> {
        // println!("CALL PRECEDURE");
        let args_value: Vec<Literal> = args
            .iter()
            .map(|value| {
                value
                    .expect_literal_r(heap)
                    .expect("It cannot be convert to Literal Value")
            })
            .collect();
        let buf = self.handel_variable_conflicts(heap);
        self.push_arguments(heap, &args_value);
        for statement in self.body.iter() {
            statement.execute(heap);
        }
        self.recovry_conflicted_variable(heap, buf);
        None
    }
}
impl Procedure {
    pub fn new(arguments: Vec<String>, body: Vec<Statement>) -> Self {
        Self { arguments, body }
    }
    fn handel_variable_conflicts(&self, heap: &mut Heap) -> HashMap<String, Literal> {
        //
        let mut buf: HashMap<String, Literal> = HashMap::new();
        for arg in self.arguments.iter() {
            let value = heap.mut_variables().remove(arg);
            if let Some(value) = value {
                buf.insert(arg.clone(), value);
            }
        }
        buf
    }
    fn recovry_conflicted_variable(&self, heap: &mut Heap, buf: HashMap<String, Literal>) {
        for (key, value) in buf.iter() {
            heap.mut_variables().insert(key.clone(), value.clone());
        }
    }
    fn push_arguments(&self, heap: &mut Heap, args: &[Literal]) {
        for (arg, value) in self.arguments.iter().zip(args.iter()) {
            heap.mut_variables().insert(arg.clone(), value.clone());
        }
    }
}
