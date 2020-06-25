use std::collections::BTreeMap;
use seed::browser::Url;
use seed::virtual_dom::ElRef;
use ulid::Ulid;
use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    pub todos: BTreeMap<Ulid, Todo>,
    pub new_todo_title: String,
    pub selected_todo: Option<SelectedTodo>,
    pub filter: Filter,
    pub base_url: Url,
}

pub struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
}

pub struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

pub enum Filter {
  All,
  Active,
  Completed,
}

// TODO: Remove
impl Model {
    pub fn add_mock_data(mut self) -> Self {
        let (id_a, id_b) = (Ulid::new(), Ulid::new());
        
        self.todos.insert(id_a, Todo {
            id: id_a,
            title: "I'm todo A".to_owned(),
            completed: false,
        });

        self.todos.insert(id_b, Todo {
            id: id_b,
            title: "I'm todo B".to_owned(),
            completed: true,
        });

        self.new_todo_title = "I'm a new todo title".to_owned();

        self.selected_todo = Some(SelectedTodo {
            id: id_b,
            title: "I'm better todo B".to_owned(),
            input_element: ElRef::new(),
        });
        self
    }
}

