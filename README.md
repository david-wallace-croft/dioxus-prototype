# Dioxus Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-prototype/blob/main/LICENSE.txt

- My first Dioxus application

## Usage

- cd dioxus-prototype/
- cargo install dioxus-cli
- dx serve --hot-reload
- Open your browser to http://localhost:8080/
- Make changes to the HTML in src/lib.rs or the CSS in public/stylesheet.css
- Note that the changes are updated in your browser as soon as you save

## Prerender

- cd dioxus-prototype/
- Temporarily make the following changes to Cargo.toml
  - These temporary changes enable prerendering but break dx serve
  - Remark out dependency dioxus-router with has no features enabled
  - Unremark dependency dioxus-router with features enabled
  - Unremark the dependencies dioxus-ssr and tokio
- Add the "ssr" feature to dioxus-router in Cargo.toml
  - Remove it when done because it currently breaks dx serve
- cargo run --bin prerender
- TODO: optionally run a command-line tool to format the generated HTML
- cp -r public/* static
- npx http-server static -o
- Revert the temporary changes to Cargo.toml

## History

- Initial release: 2022-08-21
