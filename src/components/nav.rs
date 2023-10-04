use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
    div {
      class: "app-nav box",
    nav {
    ul {
    li {
    Link {
      to: Route::Home {},
      "Home",
    }
    }
    li {
    Link {
      to: Route::Animation {},
      "Animation",
    }
    }
    li {
    Link {
      to: Route::Flashcard {},
      "Flashcard",
    }
    }
    li {
    Link {
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
      to: Route::StoryListing {},
      "Story Listing",
    }
    }
    li {
    Link {
      to: Route::Colophon {},
      "Colophon",
    }
    }
    }
    }
    }
  }
}
