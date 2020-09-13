use seed::*;
use ulid::Ulid;

use super::model::*;
use super::update::*;


pub enum Target {
    Todo (Todo),
    String(String),
    Ulid(Ulid),
}

pub struct Action {
    pub msg: Msg,
    pub target: Target,
}

#[derive(Default)]
pub struct Stack {
    stack: Vec<Action>,
    index: usize,
}

impl Stack {
    pub fn push(&mut self, action: Action ) {
        log!("Size before push: {}", self.stack.len());
        self.stack.push(action);
        self.index += 1;
        log!("Size after push: {}", self.stack.len());
    }

    pub fn pop(&mut self) -> Option<Action> {
        self.stack.pop()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }
}


