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
      "This prototype supports "
    a {
      href: "https://www.croftsoft.com/\
             library/tutorials/rust-dioxus-project-setup/",
      target: "_blank",
      "static prerendering with client-side hydration"
    }
    "."
    }
    }
  }
}
