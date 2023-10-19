use crate::components::animation::Animation;
use crate::components::colophon::Colophon;
use crate::components::flashcard::Flashcard;
use crate::components::home::Home;
use crate::components::page_layout::PageLayout;
use crate::components::page_not_found::PageNotFound;
use crate::components::retirement::Retirement;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
  #[layout(PageLayout)]
    #[route("/")]
    Home {},
    #[route("/animation")]
    Animation {},
    #[route("/colophon")]
    Colophon {},
    #[route("/flashcard")]
    Flashcard {},
    #[route("/retirement")]
    Retirement {},
  #[end_layout]
  #[route("/:..route")]
  PageNotFound {
    route: Vec<String>,
  },
}
