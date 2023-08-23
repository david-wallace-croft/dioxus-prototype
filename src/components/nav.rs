use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
    div {
      class: "app-nav box",
    span {
      class: "app-nav",
      "Navigation Sidebar",
    }
    nav {
      class: "app-nav",
    p {
    Link {
      to: Route::Home {},
      "Home",
    }
    }
    p {
    Link {
      to: Route::HighFive {},
      "High Five",
    }
    }
    p {
    a {
      href: "/manual/",
      "Manual"
    }
    }
    p {
    Link {
      to: Route::StoryListing {},
      "Story Listing",
    }
    }
    p {
    Link {
      to: Route::Colophon {},
      "Colophon",
    }
    }
    }
    }
  }
}
