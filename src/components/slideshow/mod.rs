use self::constants::{CSS, IMAGE_ASSETS};
use self::controller::Controller;
use self::user_input::UserInput;
use super::super::components::slideshow::control_panel::ControlPanel;
use ::com_croftsoft_lib_animation::web_sys::spawn_local_loop;
use ::dioxus::prelude::*;
use ::gloo_events::EventListener;
use ::tracing::debug;
use ::web_sys::Document;
use ::web_sys::wasm_bindgen::JsValue;

mod constants;
mod control_panel;
mod controller;
mod user_input;

#[allow(non_snake_case)]
#[component]
pub fn Slideshow() -> Element {
  debug!("Slideshow() render");

  let mut fullscreen_event_listener_option_signal: Signal<
    Option<EventListener>,
  > = use_signal(|| None);

  let mut fullscreen_signal: Signal<bool> = use_signal(|| false);

  let control_panel_fade_signal: Signal<bool> = use_signal(|| false);

  let control_panel_show_signal: Signal<bool> = use_signal(|| false);

  let image_source_signal: Signal<Asset> = use_signal(|| IMAGE_ASSETS[0]);

  let mut user_input_signal: Signal<UserInput> =
    use_signal(|| Default::default());

  use_drop(move || {
    debug!("dropping");

    user_input_signal.with_mut(|user_input| user_input.stop = true);
  });

  use_future(move || async move {
    let loop_updater = Controller::new(
      control_panel_fade_signal,
      control_panel_show_signal,
      image_source_signal,
      user_input_signal,
    );

    spawn_local_loop(loop_updater);
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

  let on_click_skip = move |_event: MouseEvent| {
    user_input_signal.with_mut(|user_input| user_input.skip = true);
  };

  let onmousemove = move |_event: MouseEvent| {
    user_input_signal.with_mut(|user_input| user_input.show = true);
  };

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    div {
      class: "app-fade-in-animation app-slideshow box",
      onmousemove,
    h1 {
      class: "app-title",
      "Slideshow"
    }
    div {
      id: "slideshow",
    if *control_panel_show_signal.read() {
      ControlPanel {
        fading: *control_panel_fade_signal.read(),
        fullscreen: *fullscreen_signal.read(),
        on_click_fullscreen: move |_event| fullscreen(),
        on_click_skip,
      }
    }
    img {
      src: "{image_source_signal}",
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
