use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Template(cx: Scope) -> Element {
  let nav = use_navigator(cx);
  // TODO: Remove this temporary workaround for a prerendering / hydration bug
  nav.push(Route::Home {});
  render! {
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
