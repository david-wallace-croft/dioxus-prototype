use super::super::component::footer::Footer;
use super::super::component::nav::Nav;
use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  static CSS: Asset = asset!("/public/template/stylesheet.css");

  static FAVICON: Asset = asset!("/public/template/favicon.ico");

  rsx! {
    document::Stylesheet {
      href: CSS
    }
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
