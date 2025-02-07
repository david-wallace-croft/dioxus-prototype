use crate::route::Route;
use ::dioxus::prelude::*;
use dioxus_router::prelude::Link;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> Element {
  static CSS: Asset = asset!("/assets/home/app-home.css");

  rsx! {
  document::Stylesheet {
    href: CSS
  }
  div {
    class: "app-fade-in-animation app-home box",
  h1 {
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
  ul {
  li {
  "The "
  Link {
    to: Route::Animation {},
  "Animation"
  }
  " component demonstrates"
  ul {
  li {
  "animation in a loop"
  }
  li {
  "focus, key, and mouse inputs"
  }
  }
  }
  li {
  "The "
  Link {
    to: Route::Barchart {},
  "Barchart"
  }
  " component demonstrates"
  ul {
  li {
  "using crate "
  a {
    href: "https://github.com/dioxus-community/dioxus-charts",
    target: "_blank",
  "dioxus-charts"
  }
  }
  li {
  "with CSS customization"
  }
  }
  }
  li {
  "The "
  Link {
    to: Route::Flashcard {},
  "Flashcard"
  }
  " component demonstrates"
  ul {
  li {
  "button inputs"
  }
  li {
  "opening an external webpage"
  }
  li {
  "SVG icons"
  }
  }
  }
  li {
  "The "
  a {
    href: "/manual/",
  "Manual"
  }
  " link demonstrates"
  ul {
  li {
  "integrating with pre-existing non-Dioxus webpages"
  }
  }
  }
  li {
  "The "
  Link {
    to: Route::Retirement {},
  "Retirement"
  }
  " component demonstrates"
  ul {
  li {
  "internationalization (i18n) for static prerendering"
  }
  li {
  "shared state for static prerendering"
  }
  li {
  "form inputs"
  }
  li {
  "filtering out non-numeric input values"
  }
  li {
  "displaying a value calculated from the inputs"
  }
  }
  }
  li {
  "The "
  Link {
    to: Route::Slideshow {},
  "Slideshow"
  }
  " component demonstrates"
  ul {
  li {
  "fullscreen toggle and change detection"
  }
  li {
  "loading image assets"
  }
  li {
  "periodic updates"
  }
  }
  }
  }
  }
  }
}
