
use seed::{prelude::*, *};

use super::model::*;
use super::update::*;

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
// #[allow(clippy::trivially_copy_pass_by_ref)]

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_header(),
        // This section should be hidden by default and shown when there are todos
        view_main(),
        // This footer should hidden by default and shown when there are todos
        view_footer(),
    ]
}

// ------ header ------

fn view_header() -> Node<Msg> {
    header![C!["header"],
        h1!["todos"],
        input![C!["new-todo"],
            attrs!{At::Placeholder => "What needs to be done?", At::AutoFocus => AtValue::None},
        ]
    ]
}

// ------ main ------

fn view_main() -> Node<Msg> {
    section![C!["main"],
        view_toggle_all(),
        view_todo_list(),
    ]
}

fn view_toggle_all() -> Vec<Node<Msg>> {
    vec![
        input![C!["toggle-all"], attrs!{At::Id => "toggle-all", At::Type => "checkbox"}],
        label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
    ]
}

fn view_todo_list() -> Node<Msg> {
    ul![C!["todo-list"],
        // These are here just to show the structure of the list items
        // List items should get the class `editing` when editing and `completed` when marked as completed
        li![C!["completed"],
            div![C!["view"],
                input![C!["toggle"], attrs!{At::Type => "checkbox", At::Checked => AtValue::None}],
                label!["Taste JavaScript"],
                button![C!["destroy"]],
            ],
            input![C!["edit"], attrs!{At::Value => "Create a TodoMVC template"}]
        ],
        li![
            div![C!["view"],
                input![C!["toggle"], attrs!{At::Type => "checkbox"}],
                label!["Buy a unicorn"],
                button![C!["destroy"]],
            ],
            input![C!["edit"], attrs!{At::Value => "Rule the web"}]
        ]
    ]
}

// ------ footer ------

fn view_footer() -> Node<Msg> {
    footer![C!["footer"],
        // This should be `0 items left` by default
        span![C!["todo-count"],
            strong!["0"],
            " item left",
        ],
        view_filters(),
        // Hidden if no completed items are left ↓
        button![C!["clear-completed"],
            "Clear completed"
        ]
    ]
}

fn view_filters() -> Node<Msg> {
    ul![C!["filters"],
        li![
            a![C!["selected"],
                attrs!{At::Href => "#/"},
                "All",
            ],
        ],
        li![
            a![
                attrs!{At::Href => "#/active"},
                "Active",
            ],
        ],
        li![
            a![
                attrs!{At::Href => "#/completed"},
                "Completed",
            ],
        ],
    ]
}

// `view` describes what to display.
pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        header![
            C!["header"],
            h1!["todos"],
            input![
                C!["new-todo"],
                attrs!{At::Placeholder => "What needs to be done?"},
                attrs!{At::AutoFocus => AtValue::None},
                ]
        ],
        section![
            C!["main"],
            input![
                C!["toggle-all"],
                attrs!{At::Id => "toggle-all"},
                attrs!{At::type => "checkbox"},
            ],
            label![
                attrs!{At::For => "toggle-all"},
                "Mark all as complete"
            ],
            ul![
                C!["todo-list"],
                li![
                    C!["completed"],
                    div![
                        C!["view"],
                        input![
                            C!["toggle"],
                            attrs!{At::type => "checkbox"},
                            attrs!{At::checked => AtValue::None}
                        ],
                        label!["Taste Javascript"],
                        button![
                            C!["destroy"],
                        ],
                    ],
                    input![
                        C!["edit"],
                        attrs!{At::Value => "Create a TodoMVC template"}
                    ],
                ],
                li![
                    div![
                        C!["view"],
                        input![
                            C!["toggle"],
                            attrs!{At::type => "checkbox"},
                        ],
                        label!["Buy an unicorn or not"],
                        button![
                            C!["destroy"],
                        ],
                    ],
                    input![
                        C!["edit"],
                        attrs!{At::Value => "Rule the web"}
                    ],
                ],
            ]
        ],
        // This footer should hidden by default and shown when there are todos 
        footer![
            C!["footer"],
            span![
                C!["todo-count"],
                strong!["0"],
                "item left",
            ],
            ul![
                C!["filters"],
                li![
                    a![
                        C!["selected"],
                        attrs!{At::Href => "#/"},
                        "All",
                    ],
                ],
                li![
                    a![
                        attrs!{At::Href => "#/active"},
                        "Active",
                    ],
                ],
                li![
                    a![
                        attrs!{At::Href => "#/completed"},
                        "Completed",
                    ],
                ],
            ],
            button![
                C!["clear-completed"],
                "Clear completed",
            ],
        ],
    ]
}


