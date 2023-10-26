use async_std::task::sleep;
use dioxus::prelude::*;
use std::time::Duration;

static IMAGE_PATH_PREFIX: &str = "slideshow/";
static IMAGE_NAMES: [&str; 5] = [
  "nature-a.jpg",
  "nature-b.jpg",
  "nature-c.jpg",
  "nature-d.jpg",
  "nature-e.jpg",
];

struct SlideshowState {
  image_index: usize,
  image_source: String,
}

#[allow(non_snake_case)]
pub fn Slideshow(cx: Scope) -> Element {
  let slideshow_state_use_ref: &UseRef<SlideshowState> =
    use_ref(cx, || SlideshowState {
      image_index: 0,
      image_source: make_image_source(0),
    });
  use_effect(cx, (), |()| {
    to_owned![slideshow_state_use_ref];
    async move {
      loop {
        sleep(Duration::from_millis(5_000u64)).await;
        next_image(&slideshow_state_use_ref);
      }
    }
  });
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
      onclick: move |_event| next_image(slideshow_state_use_ref),
      "Next"
    }
    }
    br {}
    img {
      src: "{slideshow_state_use_ref.read().image_source}",
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

fn next_image(slideshow_state_use_ref: &UseRef<SlideshowState>) {
  slideshow_state_use_ref.with_mut(|state| {
    // TODO: reset delay until next image is automatically loaded
    state.image_index = (state.image_index + 1) % IMAGE_NAMES.len();
    state.image_source = make_image_source(state.image_index);
  });
}
