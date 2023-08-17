use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Colophon(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      style: "text-align: center",
    "Colophon"
    }
    p {
    "This website was created using the Rust library ",
    a {
      href: "https://dioxuslabs.com/",
      target: "_blank",
    "Dioxus",
    }
    "."
    }
    p {
    "This open source repository for this website is hosted on GitHub:",
    br { },
    a {
      href: "https://github.com/david-wallace-croft/dioxus-prototype",
      target: "_blank",
    "https://github.com/david-wallace-croft/dioxus-prototype",
    }
    }
    }
  }
}
