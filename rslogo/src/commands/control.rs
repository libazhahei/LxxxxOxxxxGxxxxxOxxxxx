#![allow(unused_variables)]

use crate::heap::Heap;

use super::{Callable, Literal, Value};

pub struct PenUpCommand {}

impl Callable for PenUpCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        heap.mut_turtle().pen_up();
        None
    }
}
pub struct PenDownCommand {}

impl Callable for PenDownCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        heap.mut_turtle().pen_down();
        None
    }
}
pub struct ForwardCommand {}

impl Callable for ForwardCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let num_pixel = args.get(0)?.expect_literal_r(heap)?.expect_float()?;
        heap.mut_turtle().move_forward(num_pixel);
        None
    }
}

pub struct BackCommand {}

impl Callable for BackCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let num_pixel = args.get(0)?.expect_literal_r(heap)?.expect_float()?;
        heap.mut_turtle().move_back(num_pixel);
        None
    }
}

pub struct LeftCommand {}

impl Callable for LeftCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let num_pixel = args.get(0)?.expect_literal_r(heap)?.expect_float()?;
        heap.mut_turtle().move_left(num_pixel);
        None
    }
}
pub struct RightCommand {}

impl Callable for RightCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let num_pixel = args.get(0)?.expect_literal_r(heap)?.expect_float()?;
        heap.mut_turtle().move_right(num_pixel);
        None
    }
}
pub struct SetPenColorCommand {}

impl Callable for SetPenColorCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let color = args
            .get(0)?
            .expect_literal_r(heap)?
            .expect_int()
            .expect("Expect an integer");
        heap.mut_turtle().set_color(color);
        None
    }
}

pub struct TurnCommand {}

impl Callable for TurnCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let degree = args
            .get(0)?
            .expect_literal_r(heap)?
            .expect_int()
            .expect("Expect an integer");
        heap.mut_turtle().turn(degree);
        None
    }
}

pub struct SetHeadingCommand {}
impl Callable for SetHeadingCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let color = args
            .get(0)?
            .expect_literal_r(heap)?
            .expect_int()
            .expect("Expect an integer");
        heap.mut_turtle().set_direction(color);
        None
    }
}
pub struct SetXCommand {}
impl Callable for SetXCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let color = args.get(0)?.expect_literal_r(heap)?.expect_float()?;
        heap.mut_turtle().set_x(color);
        None
    }
}
pub struct SetYCommand {}

impl Callable for SetYCommand {
    fn call(&self, args: &[Value], heap: &mut Heap) -> Option<Literal> {
        let color = args.get(0)?.expect_literal_r(heap)?.expect_float()?;
        heap.mut_turtle().set_y(color);
        None
    }
}
