use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn StoryListing(cx: Scope) -> Element {
  let title = "TEST_TITLE";
  let by = "TEST_AUTHOR";
  let score = 0;
  let time = chrono::Utc::now();
  let comments = "TEST_COMMENTS";
  render! {
    div {
      class: "box",
    p {
      "{title} by {by} ({score}) {time} {comments}"
    }
    }
  }
}
