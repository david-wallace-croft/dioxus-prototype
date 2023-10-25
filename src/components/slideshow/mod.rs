use dioxus::prelude::*;

static IMAGE_PATH_PREFIX: &str = "slideshow/";
static IMAGE_NAMES: [&str; 5] = [
  "nature-a.jpg",
  "nature-b.jpg",
  "nature-c.jpg",
  "nature-d.jpg",
  "nature-e.jpg",
];

#[allow(non_snake_case)]
pub fn Slideshow(cx: Scope) -> Element {
  let image_index: usize = 0;
  let image_source: String = make_image_source(image_index);
  let image_index_state: &UseState<usize> = use_state(cx, || image_index);
  let image_source_state: &UseState<String> = use_state(cx, || image_source);
  render! {
    div {
      class: "app-slideshow box",
    h1 {
      class: "app-title",
      "Slideshow"
    }
    div {
      text_align: "center",
    button {
      onclick: move |_event| next_image(image_index_state, image_source_state),
      "Next"
    }
    }
    br {}
    img {
      src: "{image_source_state}",
    }
    }
  }
}

fn make_image_source(image_index: usize) -> String {
  let image_name = IMAGE_NAMES[image_index];
  let mut image_source = IMAGE_PATH_PREFIX.to_string();
  image_source.push_str(image_name);
  image_source
}

fn next_image(
  image_index_state: &UseState<usize>,
  image_source_state: &UseState<String>,
) {
  let mut image_index: usize = *image_index_state.get();
  image_index = (image_index + 1) % IMAGE_NAMES.len();
  image_index_state.set(image_index);
  let image_source = make_image_source(image_index);
  image_source_state.set(image_source);
}
