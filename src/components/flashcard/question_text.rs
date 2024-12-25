use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn QuestionText(text: &'static str) -> Element {
  rsx! {
    span {
      class: "app-question-text",
    "{text}"
    }
  }
}
