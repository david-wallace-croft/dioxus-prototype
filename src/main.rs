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

  dioxus::LaunchBuilder::new()
    // Set the server config only if we are building the server target
    .with_cfg(server_only! {
      ServeConfig::builder()
        // Enable incremental rendering
        .incremental(
          dioxus::server::IncrementalRendererConfig::new()
          // Store static files in the public directory where other static
          // assets like wasm are stored
          .static_dir(
            std::env::current_exe()
              .unwrap()
              .parent()
              .unwrap()
              .join("public")
          )
          // Don't clear the public folder on every build. The public folder
          // has other files including the wasm binary and static assets
          // required for the app to run
          .clear_cache(false)
        ).enable_out_of_order_streaming()
    })
    .launch(|| {
      rsx! {
        Router::<Route> {}
      }
    });
}
