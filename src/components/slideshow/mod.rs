use crate::components::slideshow::control_panel::ControlPanel;
use async_std::task::sleep;
use dioxus::prelude::*;
use gloo_events::EventListener;
use std::time::Duration;
use web_sys::Document;

mod control_panel;

const CONTROL_PANEL_DISPLAY_TIME: u64 = 5 * 1_000;
const IMAGE_DISPLAY_TIME: u64 = 10 * 60 * 1_000;
const POLLING_PERIOD: u64 = 100;

static IMAGE_NAMES: [&str; 5] = [
  "nature-a.jpg",
  "nature-b.jpg",
  "nature-c.jpg",
  "nature-d.jpg",
  "nature-e.jpg",
];
static IMAGE_PATH_PREFIX: &str = "/slideshow/";

struct SlideshowState {
  control_panel_time_remaining: u64,
  image_index: usize,
  image_source: String,
  image_time_remaining: u64,
}

#[allow(non_snake_case)]
pub fn Slideshow(cx: Scope) -> Element {
  let fullscreen_event_listener_option_state: &UseState<Option<EventListener>> =
    use_state(cx, || None);
  let fullscreen_state: &UseState<bool> = use_state(cx, || false);
  let slideshow_state_use_ref: &UseRef<SlideshowState> =
    use_ref(cx, || SlideshowState {
      control_panel_time_remaining: CONTROL_PANEL_DISPLAY_TIME,
      image_index: 0,
      image_source: make_image_source(0),
      image_time_remaining: IMAGE_DISPLAY_TIME,
    });
  use_future(cx, (), |_| {
    to_owned![slideshow_state_use_ref];
    async move {
      loop {
        sleep(Duration::from_millis(POLLING_PERIOD)).await;
        slideshow_state_use_ref.with_mut(|state: &mut SlideshowState| {
          state.control_panel_time_remaining = state
            .control_panel_time_remaining
            .saturating_sub(POLLING_PERIOD);
          state.image_time_remaining =
            state.image_time_remaining.saturating_sub(POLLING_PERIOD);
          if state.image_time_remaining == 0 {
            next_image(state);
          }
        });
      }
    }
  });
  use_future(cx, (), |_| {
    to_owned![fullscreen_event_listener_option_state];
    to_owned![fullscreen_state];
    async move {
      let document: Document = web_sys::window().unwrap().document().unwrap();
      if !document.fullscreen_enabled() {
        return;
      }
      let slideshow_element = document.get_element_by_id("slideshow").unwrap();
      let event_listener = EventListener::new(
        &slideshow_element,
        "fullscreenchange",
        move |_event| {
          let is_fullscreen: bool = document.fullscreen_element().is_some();
          fullscreen_state.set(is_fullscreen);
        },
      );
      fullscreen_event_listener_option_state.set(Some(event_listener));
    }
  });
  render! {
    div {
      class: "app-slideshow box",
      onmousemove: move |_event| on_mouse_move(slideshow_state_use_ref),
    h1 {
      class: "app-title",
      "Slideshow"
    }
    div {
      id: "slideshow",
    if slideshow_state_use_ref.with(|state| state.control_panel_time_remaining > 0) {
      render! {
        ControlPanel {
          fullscreen: *fullscreen_state.get(),
          on_click_fullscreen: move |_event| fullscreen(),
          on_click_skip: move |_event|
            slideshow_state_use_ref.with_mut(|state: &mut SlideshowState| {
              next_image(state);
            }),
        }
      }
    }
    img {
      src: "{slideshow_state_use_ref.with(|state| state.image_source.clone())}",
    }
    }
    }
  }
}

fn fullscreen() {
  let document: Document = web_sys::window().unwrap().document().unwrap();
  if !document.fullscreen_enabled() {
    return;
  }
  if document.fullscreen_element().is_some() {
    document.exit_fullscreen();
  } else {
    let slideshow_element = document.get_element_by_id("slideshow").unwrap();
    // log::info!("slideshow element: {slideshow_element:?}");
    let _result = slideshow_element.request_fullscreen();
  }
}

fn make_image_source(image_index: usize) -> String {
  let image_name = IMAGE_NAMES[image_index];
  let mut image_source = IMAGE_PATH_PREFIX.to_string();
  image_source.push_str(image_name);
  image_source
}

fn next_image(state: &mut SlideshowState) {
  state.image_index = (state.image_index + 1) % IMAGE_NAMES.len();
  state.image_source = make_image_source(state.image_index);
  state.image_time_remaining = IMAGE_DISPLAY_TIME;
}

fn on_mouse_move(slideshow_state_use_ref: &UseRef<SlideshowState>) {
  slideshow_state_use_ref.with_mut(|state| {
    state.control_panel_time_remaining = CONTROL_PANEL_DISPLAY_TIME;
  });
}
