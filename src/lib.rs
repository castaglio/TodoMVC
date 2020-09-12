#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod model;
mod update;
mod urls;
mod view;
mod undo;

pub use model::*;
use update::*;
use view::*;
use undo::*;

const STORAGE_KEY: &str = "lujuul-seed";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        base_url: url.to_hash_base_url(),
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::from(url),
        undo_queue: UndoQueue::default(),
        redo_queue: RedoQueue::default(),
        undo_stack: UndoStack::default(),
        redo_stack: RedoStack::default(),
    }   
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let root_element = document()
        .get_elements_by_class_name("totodo")
        .item(0)
        .expect("element with the class `totodo`");

    App::start(root_element, init, update, view);
}
