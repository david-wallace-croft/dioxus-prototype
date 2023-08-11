use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
    h1 {
      "Home Page"
    }
    p {
      "This is the home page."
    }
  }
}
