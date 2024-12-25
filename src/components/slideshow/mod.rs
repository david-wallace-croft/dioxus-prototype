use super::super::components::slideshow::control_panel::ControlPanel;
use ::dioxus::prelude::*;
use ::async_std::task::sleep;
use ::gloo_events::EventListener;
use ::std::time::Duration;
use ::web_sys::Document;

mod control_panel;

const CONTROL_PANEL_DISPLAY_TIME: u64 = 10 * 1_000;
const CONTROL_PANEL_FADE_TIME: u64 = 5 * 1_000;
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

#[derive(Clone)]
struct SlideshowState {
  control_panel_time_remaining: u64,
  image_index: usize,
  image_source: String,
  image_time_remaining: u64,
}

#[allow(non_snake_case)]
#[component]
pub fn Slideshow() -> Element {
  let fullscreen_event_listener_option_state: Signal<Option<EventListener>> = use_signal(|| None);

  let fullscreen_state: Signal<bool> = use_signal(|| false);

  let mut slideshow_state_use_ref: Signal<SlideshowState> =
    use_signal(|| SlideshowState {
      control_panel_time_remaining: CONTROL_PANEL_DISPLAY_TIME,
      image_index: 0,
      image_source: make_image_source(0),
      image_time_remaining: IMAGE_DISPLAY_TIME,
    });

  use_future(move || {
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

  use_future(move || {
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

  rsx! {
    div {
      class: "app-slideshow box",
      onmousemove: move |_event| on_mouse_move(&mut slideshow_state_use_ref),
    h1 {
      class: "app-title",
      "Slideshow"
    }
    div {
      id: "slideshow",
    if slideshow_state_use_ref.with(|state| state.control_panel_time_remaining > 0) {
      ControlPanel {
        fading: slideshow_state_use_ref.with(|state|
          state.control_panel_time_remaining < CONTROL_PANEL_FADE_TIME),
        fullscreen: *fullscreen_state.read(),
        on_click_fullscreen: move |_event| fullscreen(),
        on_click_skip: move |_event|
          slideshow_state_use_ref.with_mut(|state: &mut SlideshowState| {
            next_image(state);
          }),
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

fn on_mouse_move(slideshow_state_use_ref: &mut Signal<SlideshowState>) {
  // TODO: Can we simplify?

  let mut slideshow_state: SlideshowState = slideshow_state_use_ref.read().clone();

  slideshow_state.control_panel_time_remaining = CONTROL_PANEL_DISPLAY_TIME;

  slideshow_state_use_ref.set(slideshow_state)
}
