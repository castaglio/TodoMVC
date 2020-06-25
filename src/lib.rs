#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use std::collections::BTreeMap;
use ulid::Ulid;

mod model;

use model::model::*;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todos: BTreeMap::new(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::All,
        base_url: Url::new(),
    }.add_mock_data()
}


// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    NewTodoTitleChanged(String),

    // ------ Basic Todo operations ------

    CreateTodo,
    ToggleTodo(Ulid),
    RemoveTodo(Ulid),
    
    // ------ Bulk operations ------

    CheckOrUncheckAll,
    ClearCompleted,
    
    // ------ Selection ------

    SelectTodo(Option<Ulid>),
    SelectedTodoTitleChanged(String),
    SaveSelectedTodo,
}


// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }
        Msg::NewTodoTitleChanged(title) => {
            log!("NewTodoTitleChanged", title);
        }
    
        // ------ Basic Todo operations ------

        Msg::CreateTodo => {
            log!("CreateTodo");
        }
        Msg::ToggleTodo(id) => {
            log!("ToggleTodo");
        }
        Msg::RemoveTodo(id) => {
            log!("RemoveTodo");
        }
        
        // ------ Bulk operations ------

        Msg::CheckOrUncheckAll => {
            log!("CheckOrUncheckAll");
        }
        Msg::ClearCompleted => {
            log!("ClearCompleted");
        }
        
        // ------ Selection ------

        Msg::SelectTodo(opt_id) => {
            log!("SelectTodo", opt_id);
        },
        Msg::SelectedTodoTitleChanged(title) => {
            log!("SelectedTodoTitleChanged", title);
        },
        Msg::SaveSelectedTodo => {
            log!("SaveSelectedTodo");
        }
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    raw![include_str!("../template.html")]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    
    let root_element = document()
        .get_elements_by_class_name("todoapp")
        .item(0)
        .expect("element with the class `todoapp`");

    App::start(root_element, init, update, view);
}
