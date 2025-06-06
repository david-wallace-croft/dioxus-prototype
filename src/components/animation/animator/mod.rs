use self::square_sprite::SquareSprite;
use super::constants::{
  CONTEXT_ID_2D, FILL_STYLE_BACKGROUND, FILL_STYLE_FOREGROUND, FONT,
  FRAME_PERIOD_MILLIS_TARGET, FRAME_PERIOD_MILLIS_THRESHOLD, MESSAGE_CONTROLS,
  MESSAGE_START, MILLISECONDS_PER_SECOND,
};
use super::frame_rater_updater_input::FrameRaterUpdaterInput;
use super::user_input::UserInput;
use ::com_croftsoft_lib_animation::frame_rater::FrameRater;
use ::com_croftsoft_lib_animation::frame_rater::simple::SimpleFrameRater;
use ::com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdater;
use ::com_croftsoft_lib_animation::metronome::Metronome;
use ::com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use ::com_croftsoft_lib_role::Updater;
use ::std::cell::RefCell;
use ::std::rc::Rc;
use ::tracing::debug;
use ::web_sys::wasm_bindgen::JsCast;
use ::web_sys::wasm_bindgen::JsValue;
use ::web_sys::{
  CanvasRenderingContext2d, Document, HtmlCanvasElement, Window, window,
};

mod square_sprite;

pub struct Animator {
  canvas_height: f64,
  canvas_rendering_context_2d: CanvasRenderingContext2d,
  canvas_width: f64,
  click_count: usize,
  click_count_text: String,
  frames_per_second: String,
  frame_rater: Rc<RefCell<dyn FrameRater>>,
  frame_rater_updater: Rc<RefCell<dyn Updater>>,
  frame_rater_updater_input: Rc<RefCell<FrameRaterUpdaterInput>>,
  /// The timestamp of the current animation frame in milliseconds
  frame_time_millis_new: f64,
  /// The timestamp of the previous animation frame in milliseconds
  frame_time_millis_old: f64,
  message: &'static str,
  metronome: DeltaMetronome,
  running: bool,
  square_sprite: SquareSprite,
}

impl Animator {
  pub fn new(canvas_id: &str) -> Self {
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
        .get_context(CONTEXT_ID_2D)
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

    let frame_rater_updater: Rc<RefCell<FrameRaterUpdater>> =
      Rc::new(RefCell::new(FrameRaterUpdater::new(
        true,
        frame_rater.clone(),
        frame_rater_updater_input.clone(),
      )));

    let metronome = DeltaMetronome {
      period_millis: MILLISECONDS_PER_SECOND,
      time_millis_next_tick: 0.,
    };

    let square_size = 100.0_f64.min(canvas_width / 2.).min(canvas_height / 2.);

    let square_sprite: SquareSprite =
      SquareSprite::new(canvas_width, canvas_height, square_size);

    Self {
      canvas_height,
      canvas_rendering_context_2d,
      canvas_width,
      click_count: 0,
      click_count_text: "Clicks: 0".to_string(),
      frame_rater,
      frame_rater_updater,
      frame_rater_updater_input,
      frame_time_millis_new: 0.,
      frame_time_millis_old: 0.,
      frames_per_second: "Frames per second:".to_string(),
      message: MESSAGE_START,
      metronome,
      running: true,
      square_sprite,
    }
  }

  fn paint(&self) {
    self
      .canvas_rendering_context_2d
      .set_fill_style_str(FILL_STYLE_BACKGROUND);

    self.canvas_rendering_context_2d.fill_rect(
      0.,
      0.,
      self.canvas_width,
      self.canvas_height,
    );

    self.square_sprite.paint(&self.canvas_rendering_context_2d);

    self.canvas_rendering_context_2d.set_font(FONT);

    self
      .canvas_rendering_context_2d
      .set_fill_style_str(FILL_STYLE_FOREGROUND);

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

  fn set_message(
    &mut self,
    message: &'static str,
  ) {
    self.message = message;
  }

  pub fn update(
    &mut self,
    frame_time_millis: f64,
    user_input: &UserInput,
  ) {
    self.frame_time_millis_old = self.frame_time_millis_new;

    self.frame_time_millis_new = frame_time_millis;

    let mut delta_time: f64 =
      self.frame_time_millis_new - self.frame_time_millis_old;

    if delta_time >= FRAME_PERIOD_MILLIS_THRESHOLD {
      delta_time = FRAME_PERIOD_MILLIS_TARGET;
    }

    let mut repaint: bool = false;

    let mut update: bool = false;

    self
      .frame_rater_updater_input
      .borrow_mut()
      .update_time_millis = frame_time_millis;

    self.frame_rater_updater.borrow().update();

    if self.metronome.tick(frame_time_millis) {
      self.frames_per_second = format!(
        "Frames per second: {:.3}",
        self.frame_rater.borrow().get_frames_per_second_sampled()
      );

      repaint = true;
    }

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

      // TODO: Pass in canvas or pass in canvas bounds to permit resizing

      self.square_sprite.update(delta_time, user_input);

      repaint = true;
    }

    if repaint {
      self.paint();
    }
  }
}
