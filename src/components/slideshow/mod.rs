use crate::components::slideshow::control_panel::ControlPanel;
use async_std::task::sleep;
use dioxus::prelude::*;
use std::time::Duration;
use web_sys::Document;

mod control_panel;

const DISPLAY_PERIOD: u64 = 5_000u64;
const POLLING_PERIOD: u64 = 100u64;

static IMAGE_NAMES: [&str; 5] = [
  "nature-a.jpg",
  "nature-b.jpg",
  "nature-c.jpg",
  "nature-d.jpg",
  "nature-e.jpg",
];
static IMAGE_PATH_PREFIX: &str = "slideshow/";

struct SlideshowState {
  fullscreen: bool,
  image_index: usize,
  image_source: String,
  time_remaining: u64,
}

#[allow(non_snake_case)]
pub fn Slideshow(cx: Scope) -> Element {
  let slideshow_state_use_ref: &UseRef<SlideshowState> =
    use_ref(cx, || SlideshowState {
      fullscreen: false,
      image_index: 0,
      image_source: make_image_source(0),
      time_remaining: DISPLAY_PERIOD,
    });
  use_effect(cx, (), |()| {
    to_owned![slideshow_state_use_ref];
    async move {
      loop {
        sleep(Duration::from_millis(POLLING_PERIOD)).await;
        slideshow_state_use_ref.with_mut(|state| {
          state.time_remaining =
            state.time_remaining.saturating_sub(POLLING_PERIOD);
        });
        if slideshow_state_use_ref.read().time_remaining == 0 {
          next_image(&slideshow_state_use_ref);
        }
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
      id: "slideshow",
    ControlPanel {
      fullscreen: slideshow_state_use_ref.read().fullscreen,
      on_click_fullscreen: move |_event| fullscreen(slideshow_state_use_ref),
      on_click_skip: move |_event| next_image(slideshow_state_use_ref),
    }
    img {
      src: "{slideshow_state_use_ref.read().image_source}",
    }
    }
    }
  }
}

fn fullscreen(slideshow_state_use_ref: &UseRef<SlideshowState>) {
  let document: Document = web_sys::window().unwrap().document().unwrap();
  if !document.fullscreen_enabled() {
    return;
  }
  if document.fullscreen_element().is_some() {
    slideshow_state_use_ref.with_mut(|state| {
      state.fullscreen = false;
    });
    document.exit_fullscreen();
  } else {
    slideshow_state_use_ref.with_mut(|state| {
      state.fullscreen = true;
    });
    let _result = document
      .get_element_by_id("slideshow")
      .unwrap()
      .request_fullscreen();
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
    state.image_index = (state.image_index + 1) % IMAGE_NAMES.len();
    state.image_source = make_image_source(state.image_index);
    state.time_remaining = DISPLAY_PERIOD;
  });
}
