use super::super::components::footer::Footer;
use super::super::components::nav::Nav;
// use super::super::components::translator::SharedState;
use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  // use_shared_state_provider(cx, || SharedState {
  //   lang: "en".to_string(),
  // });
  rsx! {
    div {
      class: "frame",
    div {
      class: "col-1",
    Nav { }
    }
    div {
      class: "col-2",
    Outlet::<Route> {}
    }
    div {
      class: "col-3",
    Footer {}
    }
    }
  }
}
