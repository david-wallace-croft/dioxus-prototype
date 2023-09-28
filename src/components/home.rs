use crate::components::flashcard::flashcard::Flashcard;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      class: "app-home",
      "CroftSoft Dioxus Prototype"
    }
    p {
      "This prototype supports static prerendering with static hydration."
    }
    // Temporarily putting Flashcard on the home page to make development easier
    Flashcard {
    }
    }
  }
}
