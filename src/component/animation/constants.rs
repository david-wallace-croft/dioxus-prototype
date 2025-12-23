use ::dioxus::prelude::*;

pub const CANVAS_BACKGROUND_COLOR: &str = FILL_STYLE_BACKGROUND;

pub const CANVAS_CURSOR: &str = "crosshair";

pub const CANVAS_HEIGHT: &str = "360";

pub const CANVAS_ID: &str = "home-page-canvas";

pub const CANVAS_WIDTH: &str = "470";

pub const CONTEXT_ID_2D: &str = "2d";

pub static CSS: Asset = asset!("/public/animation/app-animation.css");

pub const FILL_STYLE_BACKGROUND: &str = "black";

pub const FILL_STYLE_FOREGROUND: &str = "white";

pub const FONT: &str = "30px Verdana";

pub const FRAMES_PER_SECOND_DEFAULT: f64 = 60.;

pub const FRAME_PERIOD_MILLIS_TARGET: f64 =
  MILLISECONDS_PER_SECOND / FRAMES_PER_SECOND_DEFAULT;

pub const FRAME_PERIOD_MILLIS_THRESHOLD: f64 = MILLISECONDS_PER_SECOND;

pub const MESSAGE_CONTROLS: &str = "Hold a key or scroll the mouse wheel";

pub const MESSAGE_START: &str = "Click on or tab to the canvas";

pub const MILLISECONDS_PER_SECOND: f64 = 1_000.;

pub const VELOCITY_PIXELS_PER_FRAME: f64 = 1.;

// pixels per millisecond = pixels per frame / milliseconds per frame
pub const VELOCITY_PIXELS_PER_MILLISECOND: f64 =
  VELOCITY_PIXELS_PER_FRAME / FRAME_PERIOD_MILLIS_TARGET;
