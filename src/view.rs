
use seed::{prelude::*, *};

use super::model::*;
use super::update::*;

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
pub fn view(model: &Model) -> Vec<Node<Msg>> {
    raw![include_str!("../template.html")]
}


