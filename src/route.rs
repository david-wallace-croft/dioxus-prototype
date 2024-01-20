use super::components::animation::Animation;
use super::components::colophon::Colophon;
use super::components::flashcard::Flashcard;
use super::components::home::Home;
use super::components::login::query::LoginQuerySegments;
use super::components::login::Login;
use super::components::page_not_found::PageNotFound;
use super::components::retirement::Retirement;
use super::components::slideshow::Slideshow;
use super::components::template::Template;
use ::dioxus::prelude::*;
use ::dioxus_router::prelude::*;
use ::serde::{Deserialize, Serialize};

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
    #[route("/login?:query_params")]
    Login {
      query_params: LoginQuerySegments,
    },
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
