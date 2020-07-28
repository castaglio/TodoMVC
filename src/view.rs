use seed::{prelude::*, *};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use ulid::Ulid;

use super::model::*;
use super::update::*;
use super::urls::*;

const ENTER_KEY: &str = "Enter";
const ESCAPE_KEY: &str = "Escape";

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(&model.new_todo_title),
        IF!(not(model.todos.is_empty()) => vec![
            view_main(&model.todos, model.selected_todo.as_ref(), model.filter),
            view_footer(&model.todos, model.filter, &model.base_url),
        ]),
    ]
}

// ------ header ------

fn view_header(new_todo_title: &str) -> Node<Msg> {
    header![
        C!["header"],
        h1!["todos"],
        input![
            C!["new-todo"],
            attrs! {
                At::Placeholder => "What needs to be done?",
                At::AutoFocus => AtValue::None,
                At::Value => new_todo_title,
            },
            input_ev(Ev::Input, Msg::NewTodoTitleChanged),
            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateTodo)
            }),
        ]
    ]
}

// ------ main ------

fn view_main(
    todos: &BTreeMap<Ulid, Todo>,
    selected_todo: Option<&SelectedTodo>,
    filter: Filter,
) -> Node<Msg> {
    section![
        C!["main"],
        view_toggle_all(todos),
        view_todo_list(todos, selected_todo, filter),
    ]
}

fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
    let all_completed = todos.values().all(|todo| todo.completed);
    vec![
        input![
            C!["toggle-all"],
            attrs! {
                At::Id => "toggle-all",
                At::Type => "checkbox",
                At::Checked => all_completed.as_at_value(),
            },
            ev(Ev::Change, |_| Msg::CheckOrUncheckAll),
        ],
        label![
            attrs! {
                At::For => "toggle-all"
            },
            "Mark all as complete"
        ],
    ]
}

fn view_todo_list(
    todos: &BTreeMap<Ulid, Todo>,
    selected_todo: Option<&SelectedTodo>,
    filter: Filter,
) -> Node<Msg> {
    let todos = todos.values().filter(|todo| match filter {
        Filter::All => true,
        Filter::Active => not(todo.completed),
        Filter::Completed => todo.completed,
    });
    ul![
        C!["todo-list"],
        todos.map(|todo| {
            let id = todo.id;
            let is_selected = Some(todo.id) == selected_todo.map(|selected_todo| selected_todo.id);

            li![
                C![
                    IF!(todo.completed => "completed"),
                    IF!(is_selected => "editing")
                ],
                el_key(&todo.id),
                div![
                    C!["view"],
                    input![
                        C!["toggle"],
                        attrs! {
                            At::Type => "checkbox",
                            At::Checked => todo.completed.as_at_value(),
                        },
                        ev(Ev::Change, move |_| Msg::ToggleTodo(id))
                    ],
                    label![
                        &todo.title,
                        ev(Ev::DblClick, move |_| Msg::SelectTodo(Some(id))),
                    ],
                    button![C!["destroy"], ev(Ev::Click, move |_| Msg::RemoveTodo(id)),],
                ],
                IF!(is_selected => {
                    let selected_todo = selected_todo.unwrap();
                    input![C!["edit"],
                        el_ref(&selected_todo.input_element),
                        attrs!{
                            At::Value => selected_todo.title
                        },
                        input_ev(Ev::Input, Msg::SelectedTodoTitleChanged),
                        keyboard_ev(Ev::KeyDown, |keyboard_event| {
                            Some(match keyboard_event.key().as_str() {
                                ESCAPE_KEY => Msg::SelectTodo(None),
                                ENTER_KEY => Msg::SaveSelectedTodo,
                                _ => return None
                            })
                        }),
                        ev(Ev::Blur, |_| Msg::SaveSelectedTodo),
                    ]
                }),
            ]
        }),
    ]
}

// ------ footer ------

fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter, base_url: &Url) -> Node<Msg> {
    let completed_count = todos.values().filter(|todo| todo.completed).count();
    let active_count = todos.len() - completed_count;

    footer![
        C!["footer"],
        // This should be `0 items left` by default
        span![
            C!["todo-count"],
            strong![active_count],
            format!(" item{} left", if active_count == 1 { "" } else { "s" }),
        ],
        view_filters(selected_filter, base_url),
        IF!(completed_count > 0 =>
        // Hidden if no completed items are left ↓
            button![C!["clear-completed"],
                "Clear completed",
                ev(Ev::Click, |_| Msg::ClearCompleted),
            ]
        )
    ]
}

fn view_filters(selected_filter: Filter, base_url: &Url) -> Node<Msg> {
    ul![
        C!["filters"],
        Filter::iter().map(|filter| {
            let urls = Urls::new(base_url);

            let (path, title) = match filter {
                Filter::All => (urls.home(), "All"),
                Filter::Active => (urls.active(), "Active"),
                Filter::Completed => (urls.completed(), "Completed"),
            };
            li![a![
                C![IF!(filter == selected_filter => "selected")],
                attrs! {At::Href => path},
                title,
            ],]
        })
    ]
}
