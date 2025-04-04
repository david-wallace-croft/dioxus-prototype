use super::super::components::slideshow::control_panel::ControlPanel;
use ::async_std::task::sleep;
use ::dioxus::prelude::*;
use ::gloo_events::EventListener;
use ::std::time::Duration;
use ::web_sys::Document;
use ::web_sys::wasm_bindgen::JsValue;

mod control_panel;

const CONTROL_PANEL_DISPLAY_TIME: u64 = 10 * 1_000;

const CONTROL_PANEL_FADE_TIME: u64 = 5 * 1_000;

const IMAGE_DISPLAY_TIME: u64 = 15 * 1_000;

const POLLING_PERIOD: u64 = 100;

static CSS: Asset = asset!("/assets/slideshow/app-slideshow.css");

static IMAGE_ASSETS: [Asset; 5] = [
  asset!("/assets/slideshow/nature-a.jpg"),
  asset!("/assets/slideshow/nature-b.jpg"),
  asset!("/assets/slideshow/nature-c.jpg"),
  asset!("/assets/slideshow/nature-d.jpg"),
  asset!("/assets/slideshow/nature-e.jpg"),
];

#[derive(Clone)]
struct SlideshowState {
  control_panel_time_remaining: u64,
  image_index: usize,
  image_source: Asset,
  image_time_remaining: u64,
}

#[allow(non_snake_case)]
#[component]
pub fn Slideshow() -> Element {
  let mut fullscreen_event_listener_option_signal: Signal<
    Option<EventListener>,
  > = use_signal(|| None);

  let mut fullscreen_signal: Signal<bool> = use_signal(|| false);

  let mut slideshow_state_signal: Signal<SlideshowState> =
    use_signal(|| SlideshowState {
      control_panel_time_remaining: CONTROL_PANEL_DISPLAY_TIME,
      image_index: 0,
      image_source: IMAGE_ASSETS[0],
      image_time_remaining: IMAGE_DISPLAY_TIME,
    });

  use_future(move || async move {
    loop {
      sleep(Duration::from_millis(POLLING_PERIOD)).await;

      slideshow_state_signal.with_mut(
        |slideshow_state: &mut SlideshowState| {
          slideshow_state.control_panel_time_remaining = slideshow_state
            .control_panel_time_remaining
            .saturating_sub(POLLING_PERIOD);

          slideshow_state.image_time_remaining = slideshow_state
            .image_time_remaining
            .saturating_sub(POLLING_PERIOD);

          if slideshow_state.image_time_remaining == 0 {
            next_image(slideshow_state);
          }
        },
      );
    }
  });

  use_future(move || async move {
    let document: Document = web_sys::window().unwrap().document().unwrap();

    if !document.fullscreen_enabled() {
      return;
    }

    let slideshow_element: web_sys::Element =
      document.get_element_by_id("slideshow").unwrap();

    let event_listener = EventListener::new(
      &slideshow_element,
      "fullscreenchange",
      move |_event| {
        let is_fullscreen: bool = document.fullscreen_element().is_some();
        fullscreen_signal.set(is_fullscreen);
      },
    );

    fullscreen_event_listener_option_signal.set(Some(event_listener));
  });

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    div {
      class: "app-fade-in-animation app-slideshow box",
      onmousemove: move |_event| on_mouse_move(&mut slideshow_state_signal),
    h1 {
      class: "app-title",
      "Slideshow"
    }
    div {
      id: "slideshow",
    if slideshow_state_signal.with(
      |state| state.control_panel_time_remaining > 0) {
      ControlPanel {
        fading: slideshow_state_signal.with(|state|
          state.control_panel_time_remaining < CONTROL_PANEL_FADE_TIME),
        fullscreen: *fullscreen_signal.read(),
        on_click_fullscreen: move |_event| fullscreen(),
        on_click_skip: move |_event|
          slideshow_state_signal.with_mut(|state: &mut SlideshowState| {
            next_image(state);
          }),
      }
    }
    img {
      src: "{slideshow_state_signal.with(|state| state.image_source.clone())}",
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
    let slideshow_element: web_sys::Element =
      document.get_element_by_id("slideshow").unwrap();

    let _result: Result<(), JsValue> = slideshow_element.request_fullscreen();
  }
}

fn next_image(state: &mut SlideshowState) {
  state.image_index = (state.image_index + 1) % IMAGE_ASSETS.len();

  state.image_source = IMAGE_ASSETS[state.image_index];

  state.image_time_remaining = IMAGE_DISPLAY_TIME;
}

fn on_mouse_move(slideshow_state_signal: &mut Signal<SlideshowState>) {
  // TODO: Can we simplify?

  let mut slideshow_state: SlideshowState =
    slideshow_state_signal.read().clone();

  slideshow_state.control_panel_time_remaining = CONTROL_PANEL_DISPLAY_TIME;

  slideshow_state_signal.set(slideshow_state)
}
