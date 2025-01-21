use super::super::components::footer::Footer;
use super::super::components::nav::Nav;
use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  static FAVICON: Asset = asset!("/assets/favicon.ico");

  rsx! {
    document::Link {
      href: FAVICON,
      rel: "icon",
    }
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
