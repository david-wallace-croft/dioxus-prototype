use super::component::animation::Animation;
use super::component::barchart::Barchart;
use super::component::colophon::Colophon;
use super::component::flashcard::Flashcard;
use super::component::home::Home;
use super::component::retirement::Retirement;
use super::component::slideshow::Slideshow;
use super::component::template::Template;
use ::dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
  #[layout(Template)]
  #[route("/")]
  Home {},
  #[route("/animation")]
  Animation {},
  #[route("/barchart")]
  Barchart {},
  #[route("/colophon")]
  Colophon {},
  #[route("/flashcard")]
  Flashcard {},
  #[route("/retirement")]
  Retirement {},
  #[route("/slideshow")]
  Slideshow {},
}
