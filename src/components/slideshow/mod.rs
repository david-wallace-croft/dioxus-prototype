use dioxus::prelude::*;

static IMAGE_SOURCE: &str = "slideshow/nature-a.jpg";

#[allow(non_snake_case)]
pub fn Slideshow(cx: Scope) -> Element {
  let image_source: &UseState<String> =
    use_state(cx, || IMAGE_SOURCE.to_string());
  render! {
    div {
      class: "app-slideshow box",
    h1 {
      class: "app-title",
      "Slideshow"
    }
    img {
      src: "{image_source}",
    }
    }
  }
}
