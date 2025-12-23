use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {
  rsx! {
    Router::<Route> { }
  }
}
