use crate::components::blog::Blog;
use crate::components::home::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
enum Route {
  #[route("/")]
  Home {},
  #[route("/blog")]
  Blog {},
}
