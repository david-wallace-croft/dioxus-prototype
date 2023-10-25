use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

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
        "loading image assets"
      }
      }
    }
    }
    }
  }
}
