#![allow(clippy::wildcard_imports)]

use std::convert::TryFrom;
use std::mem;

use seed::{prelude::*, *};
use ulid::Ulid;

use super::model::*;
use super::undo::*;

const STORAGE_KEY: &str = "lujuul-seed";

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    NewTodoTitleChanged(String),
    // NewTodoTitleChangedUndo,
    // NewTodoTitleChangedRedo,

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

    // ------ Undo/Redo operations ------
    Undo,
    Redo,

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
            log!("new title: {}", title);
            // let c =  model.new_todo_title_record.apply(Add(title.chars().last().unwrap()));
            // let _c = match c {
            //     Ok(applied) => applied,
            //     Err(error) => log!("Cannot undo because error: {}", error),
            // };
            
            model.new_todo_title = title;
        }

        // Msg::NewTodoTitleChangedUndo => {
        //     log!("doing undo");
        //     let c = model.new_todo_title_record.undo();
        //     let _c = match c {
        //         Ok(undo) => undo,
        //         Err(error) => log!("Cannot undo because error: {}", error),
        //     };

        //     model.new_todo_title = model.new_todo_title_record.target().to_string();
        // }

        // Msg::NewTodoTitleChangedRedo => {
        //     log!("doing redo");
        //     log!("before redo target = {}", model.new_todo_title_record.target().to_string().length());
        //     let c = model.new_todo_title_record.redo();
        //     let _c = match c {
        //         Ok(redo) => redo,
        //         Err(error) => log!("Cannot redo because error: {}", error),
        //     };
        //     log!("c = {}", _c);
        //     log!("after redo target = {}", model.new_todo_title_record.target().to_string());
        //     model.new_todo_title = model.new_todo_title_record.target().to_string();
        // }

        // ------ Basic Todo operations ------
        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                log!("New todo: {}", title);

                let id = Ulid::new();
                let todo = Todo::new(id, title.to_owned(), false, String::new());

                let action = Action {
                    msg: Msg::CreateTodo,
                    target: Target::Todo(todo),
                };

                model.undo_stack.push(action);

                let mut todos_owned = model.todos.to_owned();
                todos_owned.insert(id, todo);
                model.undo_queue.push(todos_owned);
                let new_todos = model.undo_queue.current();
                model.undo_queue.push(model.todos.to_owned());
                model.todos = new_todos;

                model.new_todo_title.clear();
            }
        }
        Msg::ToggleTodo(id) => {
            if let Some(todo) = model.todos.get_mut(&id) {
                todo.completed = not(todo.completed);
            }
        }
        Msg::RemoveTodo(id) => {
            let mut todos_owned = model.todos.to_owned();
            todos_owned.remove(&id);
            model.undo_queue.push(todos_owned);
            model.todos = model.undo_queue.current();
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

        Msg::Undo => {
            undo(model);

            let todos_owned = model.todos.to_owned();
            model.redo_queue.push(todos_owned);
            model.todos = model.undo_queue.current();
            model.undo_queue.undo();
            
        }
        Msg::Redo => {
            model.undo_queue.redo();
            model.todos = model.redo_queue.current();
        }
    }
    LocalStorage::insert(STORAGE_KEY, &model.todos).expect("save todos to LocalStorage");
}

fn undo (model: &mut Model) {
    let Some(action) = model.undo_stack.pop();

    match action.msg {
        Msg::CreateTodo => {
            let todo = (&action.target);
            let removed = model.todos.remove(action.target.id);
            model.redo_stack

        }

        _ => println!("nothing")
    }

}