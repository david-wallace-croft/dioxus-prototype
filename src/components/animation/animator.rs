use super::color::Color;
use super::frame_rater_updater_input::FrameRaterUpdaterInput;
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::frame_rater::FrameRater;
use ::com_croftsoft_lib_animation::frame_rater::simple::SimpleFrameRater;
use ::com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdater;
use ::com_croftsoft_lib_animation::metronome::Metronome;
use ::com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::com_croftsoft_lib_role::Updater;
use ::std::cell::RefCell;
use ::std::mem::take;
use ::std::rc::Rc;
use ::tracing::debug;
use ::web_sys::wasm_bindgen::JsCast;
use ::web_sys::wasm_bindgen::JsValue;
use ::web_sys::{
  CanvasRenderingContext2d, Document, HtmlCanvasElement, Window, window,
};

const FRAME_PERIOD_MILLIS_TARGET: f64 = 1_000.;
const MESSAGE_CONTROLS: &str = "Hold a key or scroll the mouse wheel";
const MESSAGE_START: &str = "Click on or tab to the canvas";

pub struct Animator {
  canvas_height: f64,
  canvas_rendering_context_2d: CanvasRenderingContext2d,
  canvas_width: f64,
  click_count: usize,
  click_count_text: String,
  color: Color,
  delta_x: f64,
  delta_y: f64,
  frames_per_second: String,
  frame_rater: Rc<RefCell<dyn FrameRater>>,
  frame_rater_updater: Rc<RefCell<dyn Updater>>,
  frame_rater_updater_input: Rc<RefCell<FrameRaterUpdaterInput>>,
  maximum_drift: u8,
  message: &'static str,
  metronome: DeltaMetronome,
  running: bool,
  square_size: f64,
  user_input: Rc<RefCell<UserInput>>,
  x: f64,
  y: f64,
}

impl Animator {
  pub fn new(
    canvas_id: &str,
    user_input: Rc<RefCell<UserInput>>,
  ) -> Self {
    let window: Window = window().expect("global window does not exists");

    let document: Document =
      window.document().expect("expecting a document on window");

    let html_canvas_element = document
      .get_element_by_id(canvas_id)
      .expect("expecting a canvas in the document")
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();

    let canvas_rendering_context_2d: CanvasRenderingContext2d =
      html_canvas_element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let canvas_height: f64 = html_canvas_element.height() as f64;

    let canvas_width: f64 = html_canvas_element.width() as f64;

    let frame_rater: Rc<RefCell<dyn FrameRater>> = Rc::new(RefCell::new(
      SimpleFrameRater::new(FRAME_PERIOD_MILLIS_TARGET),
    ));

    let frame_rater_updater_input: Rc<RefCell<FrameRaterUpdaterInput>> =
      Default::default();

    let frame_rater_updater = Rc::new(RefCell::new(FrameRaterUpdater::new(
      true,
      frame_rater.clone(),
      frame_rater_updater_input.clone(),
    )));

    let metronome = DeltaMetronome {
      period_millis: 1_000.,
      time_millis_next_tick: 0.,
    };

    Self {
      canvas_height,
      canvas_rendering_context_2d,
      canvas_width,
      click_count: 0,
      click_count_text: "Clicks: 0".to_string(),
      color: Color::random(),
      delta_x: 1.,
      delta_y: 1.,
      frame_rater,
      frame_rater_updater,
      frame_rater_updater_input,
      frames_per_second: "Frames per second:".to_string(),
      user_input,
      maximum_drift: 0,
      message: MESSAGE_START,
      metronome,
      running: true,
      square_size: 100.0_f64.min(canvas_width / 2.).min(canvas_height / 2.),
      x: -1.,
      y: -1.,
    }
  }

  fn paint(&self) {
    self.canvas_rendering_context_2d.set_fill_style_str("black");

    self.canvas_rendering_context_2d.fill_rect(
      0.,
      0.,
      self.canvas_width,
      self.canvas_height,
    );

    let fill_style: String = self.color.as_fill_style_string();

    self
      .canvas_rendering_context_2d
      .set_fill_style_str(&fill_style);

    self.canvas_rendering_context_2d.fill_rect(
      self.x,
      self.y,
      self.square_size,
      self.square_size,
    );

    self.canvas_rendering_context_2d.set_font("30px Verdana");

    self.canvas_rendering_context_2d.set_fill_style_str("white");

    let _result: Result<(), JsValue> = self
      .canvas_rendering_context_2d
      .fill_text(self.message, 4., 30.);

    let _result: Result<(), JsValue> = self
      .canvas_rendering_context_2d
      .fill_text(&self.frames_per_second, 4., self.canvas_height - 12.);

    let _result: Result<(), JsValue> = self
      .canvas_rendering_context_2d
      .fill_text(&self.click_count_text, 4., self.canvas_height - 42.);
  }

  fn adjust_maximum_drift(
    &mut self,
    delta: i8,
  ) {
    self.maximum_drift = self.maximum_drift.saturating_add_signed(delta);
  }

  fn set_message(
    &mut self,
    message: &'static str,
  ) {
    self.message = message;
  }

  fn update(&mut self) {
    // TODO: Make distance traveled proportional to time elapsed

    if self.delta_x > 0. {
      if self.x + self.delta_x + self.square_size > self.canvas_width {
        self.delta_x = -self.delta_x;
      }
    } else if self.x + self.delta_x < 0. {
      self.delta_x = -self.delta_x;
    }

    self.x += self.delta_x;

    if self.delta_y > 0. {
      if self.y + self.delta_y + self.square_size > self.canvas_height {
        self.delta_y = -self.delta_y;
      }
    } else if self.y + self.delta_y < 0. {
      self.delta_y = -self.delta_y;
    }

    self.y += self.delta_y;

    if self.maximum_drift == 0 {
      self.maximum_drift = 1;
    }

    self.color.drift(self.maximum_drift);
  }
}

impl LoopUpdater for Animator {
  fn update_loop(
    &mut self,
    update_time: f64,
  ) -> bool {
    let mut repaint = false;

    let mut update = false;

    self
      .frame_rater_updater_input
      .borrow_mut()
      .update_time_millis = update_time;

    self.frame_rater_updater.borrow().update();

    if self.metronome.tick(update_time) {
      self.frames_per_second = format!(
        "Frames per second: {:.3}",
        self.frame_rater.borrow().get_frames_per_second_sampled()
      );

      repaint = true;
    }

    // Take the user input and replace it with the default values to reset

    let user_input: UserInput = take(&mut *self.user_input.borrow_mut());

    if user_input.blur {
      // debug!("blur");

      self.set_message(MESSAGE_START);

      self.running = true;
    }

    if user_input.click {
      self.click_count += 1;

      self.click_count_text = format!("Clicks: {}", self.click_count);

      repaint = true;
    }

    let delta: i8 = user_input.drift;

    if delta != 0 {
      // debug!("delta: {delta}");

      self.adjust_maximum_drift(delta);

      update = true;
    }

    if user_input.focus {
      self.set_message(MESSAGE_CONTROLS);

      repaint = true;

      self.running = false;
    }

    if user_input.play {
      debug!("play requested");

      self.running = true;
    }

    if user_input.pause {
      debug!("pause requested");

      self.running = false;
    }

    if self.running || update {
      // debug!("running: {running}, update: {update}");

      self.update();

      repaint = true;
    }

    if repaint {
      self.paint();
    }

    let stopping: bool = user_input.stop;

    if stopping {
      debug!("stopping");
    }

    stopping
  }
}
