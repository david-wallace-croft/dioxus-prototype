use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
  render! {
    app_high_five { }
    app_story_listing { }
  }
}

pub fn app_high_five(cx: Scope) -> Element {
  let mut count = use_state(&cx, || 0);
  render! {
    div {
      class: "app-high-five",
      h1 {
        "High-Five counter: {count}",
      }
      button {
        class: "app-high-five",
        onclick: move |_| count -= 1,
        "Down low!",
      }
      button {
        class: "app-high-five",
        onclick: move |_| count += 1,
        "Up high!",
      }
    }
  }
}

pub fn app_story_listing(cx: Scope) -> Element {
  let title = "TEST_TITLE";
  let by = "TEST_AUTHOR";
  let score = 0;
  let time = chrono::Utc::now();
  let comments = "TEST_COMMENTS";
  render! {
    p {
      "{title} by {by} ({score}) {time} {comments}"
    }
  }
}
