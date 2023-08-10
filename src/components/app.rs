use super::high_five::HighFive;
use super::story_listing::StoryListing;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  render! {
    HighFive { }
    StoryListing { }
  }
}
