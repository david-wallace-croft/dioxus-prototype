use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
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
      to: Route::Flashcard {},
      "Flashcard",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::HighFive {},
      "High Five",
    }
    }
    li {
    a {
      href: "/manual/",
      "Manual"
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::StoryListing {},
      "Story Listing",
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
