use crate::components::animation::Animation;
use crate::components::colophon::Colophon;
use crate::components::flashcard::Flashcard;
use crate::components::home::Home;
use crate::components::login::Login;
use crate::components::page_not_found::PageNotFound;
use crate::components::retirement::Retirement;
use crate::components::slideshow::Slideshow;
use crate::components::template::Template;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
  #[layout(Template)]
    #[route("/")]
    Home {},
    #[route("/animation")]
    Animation {},
    #[route("/colophon")]
    Colophon {},
    #[route("/flashcard")]
    Flashcard {},
    #[route("/login")]
    Login {},
    #[route("/retirement")]
    Retirement {},
    #[route("/slideshow")]
    Slideshow {},
  #[end_layout]
  #[route("/:..route")]
  PageNotFound {
    route: Vec<String>,
  },
}
