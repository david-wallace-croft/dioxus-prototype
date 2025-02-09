use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Nav() -> Element {
  static CSS: Asset = asset!("/assets/nav/app-nav.css");

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    nav {
      class: "app-nav box",
    ul {
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Home {},
    "Home",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Animation {},
    "Animation",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Barchart {},
    "Barchart",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Flashcard {},
    "Flashcard",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Retirement {},
    "Retirement",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Slideshow {},
    "Slideshow",
    }
    }
    li {
    a {
      href: "/merged/",
    "Merged"
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Colophon {},
    "Colophon",
    }
    }
    }
    }
  }
}
