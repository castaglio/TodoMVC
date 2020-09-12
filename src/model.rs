use seed::browser::Url;
use seed::prelude::*;
use seed::virtual_dom::ElRef;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum_macros::EnumIter;
use ulid::Ulid;

use super::undo::*;

pub const ACTIVE: &str = "active";
pub const COMPLETED: &str = "completed";

// ------ ------
//     Model
// ------ ------


// `Model` describes our app state.
pub struct Model {
    pub base_url: Url,
    pub todos: BTreeMap<Ulid, Todo>,
    pub new_todo_title: String,
    pub selected_todo: Option<SelectedTodo>,
    pub filter: Filter,
    pub undo_queue: UndoQueue,
    pub redo_queue: RedoQueue,
    pub undo_stack: UndoStack,
    pub redo_stack: RedoStack,
}

impl Model {
    pub fn createtodo(&mut self) {
        self.todos = self.undo_queue.current();
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Todo {
    pub id: Ulid,
    pub title: String,
    pub completed: bool,
    pub markdown: String,
}

impl Todo {
    pub fn new(id: Ulid, title: String, completed: bool, markdown: String) -> Todo {
        Todo {
            id, title, completed, markdown,
        }
    }
}

pub struct SelectedTodo {
    pub id: Ulid,
    pub title: String,
    pub input_element: ElRef<web_sys::HtmlInputElement>,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl From<Url> for Filter {
    fn from(mut url: Url) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            [ACTIVE] => Self::Active,
            [COMPLETED] => Self::Completed,
            _ => Self::All,
        }
    }
}
