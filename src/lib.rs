use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
  let mut count = use_state(&cx, || 0);

  cx.render(rsx!(
    div {
      class: "justify-center p-2 mt-5",
      h1 {
        class: "mb-8 text-4xl font-light",
        "High-Five counter: {count}",
      }
      button {
        class: "mb-4 mr-2 text-white bg-blue-500 border-0 rounded py-1 px-4
          focus:outline-none hover:bg-gray-300",
        onclick: move |_| count -= 1,
        "Down low!",
      }
      button {
        class: "mb-4 text-white bg-blue-500 border-0 rounded py-1 px-4
          focus:outline-none hover:bg-gray-300",
        onclick: move |_| count += 1,
        "Up high!",
      }
    }
  ))
}
