use crate::prelude::*;

use crate::pages::{Home, NotFound};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(dead_code)]
pub enum Route {
    #[layout(crate::components::layout::Layout)]
        #[route("/")]
        #[allow(non_snake_case)]
        Home {},

    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
