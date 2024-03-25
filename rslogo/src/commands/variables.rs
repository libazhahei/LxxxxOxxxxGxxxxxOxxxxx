use crate::heap::Heap;
use std::ops::Add;

use super::ValueType;
use super::{Assignable, Value};

pub struct MakeCommand {}
impl Assignable for MakeCommand {
    fn declare(&self, left: &str, right: &Value, heap: &mut Heap) {
        let variable_name = left.clone();
        let variable_value = right.expect_literal_r(heap).unwrap();
        heap.mut_variables()
            .insert(variable_name.to_string(), variable_value);
    }
}

pub struct AddAssignCommand {}
impl Assignable for AddAssignCommand {
    fn declare(&self, left: &str, right: &Value, heap: &mut Heap) {
        let variable_value: ValueType = right.expect_literal_r(heap).unwrap();
        let (variable_name, curr_value) = heap
            .mut_variables()
            .remove_entry(left)
            .expect("Variable does not exist");
        let curr_value = curr_value.add(variable_value);
        heap.mut_variables()
            .insert(variable_name.to_string(), curr_value);
    }
}
