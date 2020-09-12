use seed::{prelude::*,*};
use std::collections::BTreeMap;
use ulid::Ulid;

use super::model::*;
use super::update::*;


pub trait TTarget {
}

impl TTarget for Todo {

}

impl TTarget for Ulid {

}

impl TTarget for String {

}

// pub enum Target {
//     Todo (Todo),
//     String(String),
//     Ulid(Ulid),
// }

pub struct Action {
    pub msg: Msg,
    pub target: dyn TTarget,
}

//impl<T> TTarget for Vec<T> {}

#[derive(Default)]
pub struct UndoStack {
    stack: Vec<Box<Action>>,
    index: usize,
}

impl UndoStack {

    pub fn push(&mut self, action: Box<Action> ) {
        log!("Size before push: {}", self.stack.len());
        self.stack.push(action);
        self.index += 1;
        log!("Size after push: {}", self.stack.len());
    }

    pub fn pop(&mut self) -> Option<Box<Action>> {
        self.stack.pop()
    }

    // pub fn pop(&mut self) -> Option<Box<Action>> {
    //     let Some(action_undo) = self.stack.pop();
    //     match action_undo.msg {
    //         Msg::CreateTodo => {
    //             let target = &action_undo.target;
    //             match target {
    //                 Target::Todo(todo) => {
    //                     let new_action = Box::new(Action {
    //                                                         msg: Msg::RemoveTodo(todo.id),
    //                                                         target: Ulid(todo.id),
    //                                                 });
    //                     Some(new_action)
    //                 }
    //                 _ => None
    //             }
    //         }
    //         _ => None
    //     }
    // }
}

#[derive(Default)]
pub struct RedoStack {
    stack: Vec<Box<Action>>,
    index: usize,
}

#[derive(Default)]
pub struct UndoQueue {
    queue: Vec<BTreeMap<Ulid,Todo>>,
    index: usize,
}

#[derive(Default)]
pub struct RedoQueue {
    queue: Vec<BTreeMap<Ulid,Todo>>,
    index: usize,
}



impl UndoQueue {
    pub fn new(entry: BTreeMap<Ulid,Todo>) -> UndoQueue {
        UndoQueue { 
            queue: vec![entry],
            index: 0,
        }
    }

    pub fn current(&mut self) -> BTreeMap<Ulid,Todo> {
        log!("Size before pop: {}", self.queue.len());
        self.queue.pop().unwrap_or_default()
    }

    pub fn get_current_index(&self) -> usize {
        self.index
    }

    pub fn push(&mut self, entry: BTreeMap<Ulid,Todo>) {
        log!("Size before push: {}", self.queue.len());
        self.queue.push(entry);
        self.index += 1;
        log!("Size after push: {}", self.queue.len());
    }

    pub fn undo(&mut self) {
        if self.index >= 1 {
            self.index -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.index < (self.queue.len() - 1) {
            self.index += 1;
        }
    }
}

impl RedoQueue {
    pub fn new(entry: BTreeMap<Ulid,Todo>) -> RedoQueue {
        RedoQueue {
            queue: vec![entry],
            index: 0,
        }
    }

    pub fn current(&mut self) -> BTreeMap<Ulid,Todo> {
        self.queue.pop().unwrap_or_default()
    }

    pub fn push(&mut self, entry: BTreeMap<Ulid,Todo>) {
        self.queue.push(entry);
    }
}


// struct InternalState {
//     undo_queue: UndoQueue<Todo>
// }

// impl InternalState {
//     pub fn new(id: Ulid, title: String, completed: bool, markdown: String) -> InternalState {
//         InternalState {
//             undo_queue: UndoQueue::new(Todo {
//                 id: id,
//                 title: title,
//                 completed: completed,
//                 markdown: markdown
//             })
//         }
//     }

//     pub fn todo(&self) -> Todo {
//         self.undo_queue.current()
//     }

//     pub fn undo(&mut self) {
//         self.undo_queue.undo();
//     }

//     pub fn redo(&mut self) {
//         self.undo_queue.redo();
//     }
// }   