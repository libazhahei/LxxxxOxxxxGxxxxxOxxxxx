use std::rc::Rc;

use crate::{
    ast::Callee,
    commands::{
        control::{
            BackCommand, ForwardCommand, LeftCommand, PenDownCommand, PenUpCommand, RightCommand,
            SetHeadingCommand, SetPenColorCommand, SetXCommand, SetYCommand, TurnCommand,
        },
        queries::{ColorCommand, HeadingCommand, XCorCommand, YCorCommand},
    },
    tokens::{AssignmentType, CommandType, QueryType},
};

use super::{
    variables::{AddAssignCommand, MakeCommand},
    Assignable, AssignableFactory, Callable, CallableFactory,
};

impl CallableFactory {
    pub fn build(callee: &Callee) -> Option<Rc<dyn Callable>> {
        match callee {
            Callee::Command(command) => match command {
                CommandType::Penup => Some(Rc::new(PenUpCommand {})),
                CommandType::Forward => Some(Rc::new(ForwardCommand {})),
                CommandType::Pendown => Some(Rc::new(PenDownCommand {})),
                CommandType::Back => Some(Rc::new(BackCommand {})),
                CommandType::Left => Some(Rc::new(LeftCommand {})),
                CommandType::Right => Some(Rc::new(RightCommand {})),
                CommandType::Setheading => Some(Rc::new(SetHeadingCommand {})),
                CommandType::Setpencolor => Some(Rc::new(SetPenColorCommand {})),
                CommandType::Turn => Some(Rc::new(TurnCommand {})),
                CommandType::Setx => Some(Rc::new(SetXCommand {})),
                CommandType::Sety => Some(Rc::new(SetYCommand {})),
            },
            Callee::Query(query) => match query {
                QueryType::Color => Some(Rc::new(ColorCommand {})),
                QueryType::Heading => Some(Rc::new(HeadingCommand {})),
                QueryType::Xcor => Some(Rc::new(XCorCommand {})),
                QueryType::Ycor => Some(Rc::new(YCorCommand {})),
            },
            _ => None,
        }
    }
}

impl AssignableFactory {
    pub fn build(assign_type: &AssignmentType) -> Option<Rc<dyn Assignable>> {
        match assign_type {
            AssignmentType::Make => Some(Rc::new(MakeCommand {})),
            AssignmentType::Addassign => Some(Rc::new(AddAssignCommand {})),
        }
    }
}
