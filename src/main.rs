use ::dioxus::logger;
use ::dioxus::prelude::*;
use ::dioxus_prototype::route::Route;
use ::tracing::{Level, info};

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
  Ok(
    Route::static_routes()
      .into_iter()
      .map(|route| route.to_string())
      .collect::<Vec<_>>(),
  )
}

fn main() {
  // WARNING: Setting level to TRACE causes a problem with pre-rendering
  logger::init(Level::INFO).expect("Failed to initialize logger");

  info!("CroftSoft Dioxus Prototype v{}", env!("CARGO_PKG_VERSION"));

  LaunchBuilder::new()
    .with_cfg(server_only! {
      ServeConfig::builder()
        .incremental(IncrementalRendererConfig::new())
        .build()
        .unwrap()
    })
    .launch(|| {
      rsx! {
        Router::<Route> {}
      }
    })
}
