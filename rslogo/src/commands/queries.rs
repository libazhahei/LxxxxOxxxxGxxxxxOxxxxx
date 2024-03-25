#![allow(unused_variables)]

use crate::heap::Heap;

use super::Value;
use super::{Callable, Literal};

pub struct XCorCommand {}

impl Callable for XCorCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        Some(Literal::Float(heap.turtle().x()))
    }
}

pub struct YCorCommand {}

impl Callable for YCorCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        Some(Literal::Float(heap.turtle().y()))
    }
}
pub struct HeadingCommand {}

impl Callable for HeadingCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        Some(Literal::Int(heap.turtle().direction()))
    }
}

pub struct ColorCommand {}

impl Callable for ColorCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        Some(Literal::Int(heap.turtle().color()))
    }
}
