use ::dioxus::prelude::*;

pub const CONTROL_PANEL_DISPLAY_TIME: f64 = 10. * MILLISECONDS_PER_SECOND;

pub const CONTROL_PANEL_FADE_TIME: f64 = 5. * MILLISECONDS_PER_SECOND;

pub static CSS: Asset = asset!("/public/slideshow/app-slideshow.css");

pub static IMAGE_ASSETS: [Asset; 5] = [
  asset!("/public/slideshow/nature-a.jpg"),
  asset!("/public/slideshow/nature-b.jpg"),
  asset!("/public/slideshow/nature-c.jpg"),
  asset!("/public/slideshow/nature-d.jpg"),
  asset!("/public/slideshow/nature-e.jpg"),
];

pub const IMAGE_DISPLAY_TIME: f64 = 15. * MILLISECONDS_PER_SECOND;

pub const MILLISECONDS_PER_SECOND: f64 = 1_000.;
