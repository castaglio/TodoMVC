#![allow(clippy::wildcard_imports)]

use std::convert::TryFrom;
use std::mem;

use seed::prelude::*;
use ulid::Ulid;

use super::model::*;

const STORAGE_KEY: &str = "todos-seed";

// ------ ------
//    Update
// ------ ------

pub enum Msg {
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
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.filter = match url.remaining_hash_path_parts().as_slice() {
                [ACTIVE] => Filter::Active,
                [COMPLETED] => Filter::Completed,
                _ => Filter::All,
            };
        }
        Msg::NewTodoTitleChanged(title) => {
            model.new_todo_title = title;
        }

        // ------ Basic Todo operations ------
        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                let id = Ulid::new();
                model.todos.insert(
                    id,
                    Todo {
                        id,
                        title: title.to_owned(),
                        completed: false,
                    },
                );
                model.new_todo_title.clear();
            }
        }
        Msg::ToggleTodo(id) => {
            if let Some(todo) = model.todos.get_mut(&id) {
                todo.completed = not(todo.completed);
            }
        }
        Msg::RemoveTodo(id) => {
            model.todos.remove(&id);
        }

        // ------ Bulk operations ------
        Msg::CheckOrUncheckAll => {
            let all_checked = model.todos.values().all(|todo| todo.completed);
            for todo in model.todos.values_mut() {
                todo.completed = not(all_checked);
            }
        }
        Msg::ClearCompleted => {
            // TODO: Refactor with `BTreeMap::drain_filter` once stable.
            model.todos = mem::take(&mut model.todos)
                .into_iter()
                .filter(|(_, todo)| not(todo.completed))
                .collect();
        }

        // ------ Selection ------
        Msg::SelectTodo(Some(opt_id)) => {
            if let Some(todo) = model.todos.get(&opt_id) {
                let input_element = ElRef::new();

                model.selected_todo = Some(SelectedTodo {
                    id: opt_id,
                    title: todo.title.clone(),
                    input_element: input_element.clone(),
                });

                let title_length = u32::try_from(todo.title.len()).expect("title length as u32");
                orders.after_next_render(move |_| {
                    let input_element = input_element.get().expect("input_element");

                    input_element.focus().expect("focus input_element");

                    input_element
                        .set_selection_range(title_length, title_length)
                        .expect("move cursor to the end of input_element");
                });
            }
        }
        Msg::SelectTodo(None) => {
            model.selected_todo = None;
        }
        Msg::SelectedTodoTitleChanged(title) => {
            if let Some(selected_todo) = &mut model.selected_todo {
                selected_todo.title = title;
            }
        }
        Msg::SaveSelectedTodo => {
            if let Some(selected_todo) = model.selected_todo.take() {
                if let Some(todo) = model.todos.get_mut(&selected_todo.id) {
                    todo.title = selected_todo.title;
                }
            }
        }
    }
    LocalStorage::insert(STORAGE_KEY, &model.todos).expect("save todos to LocalStorage");
}
