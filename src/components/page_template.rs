use crate::components::footer::Footer;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn PageTemplate(cx: Scope) -> Element {
  render! {
    div {
      class: "frame",
    div {
      class: "col-1",
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
