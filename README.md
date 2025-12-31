# Dioxus Prototype

[![MIT licensed][mit-badge]][mit-url]
[![Rust][status-badge]][status-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-prototype/blob/main/LICENSE.txt
[status-badge]: https://github.com/david-wallace-croft/dioxus-prototype/actions/workflows/rust.yml/badge.svg
[status-url]: https://github.com/david-wallace-croft/dioxus-prototype/actions/workflows/rust.yml

- My first Dioxus application
- Makes a Content Delivery Network (CDN)-compatible static HTML distribution
  - Includes static prerendering with client-side hydration

## Online Demonstration

- An online demonstration is available at this website:
  - https://www.persentia.com/

![CroftSoft Dioxus Prototype 2023-10-20](./media/dioxus-prototype-2023-10-20-a.png)

## Tutorial

- Rust-Dioxus Project Setup
  - https://www.croftsoft.com/library/tutorials/rust-dioxus-project-setup/

## Utilities Installation

- Install Rust
  - https://www.rust-lang.org/
- Install the Dioxus Command Line Interface (CLI) "dx"
  - cargo install dioxus-cli
  - https://github.com/DioxusLabs/dioxus/tree/master/packages/cli
- Install npm
  - npm installs utilities such as prettier
  - npm scripts run the dx and cargo commands
  - npm can be installed by installing node.js
  - https://nodejs.org/

## Quick Start

- cd dioxus-prototype/
- npm install
- npm test

## Hot Reload

- cd dioxus-prototype/
- npm install
  - Installs the utility http-server to serve the HTML
  - Installs the utility prettier to format the HTML
  - Installs the utility shx to move files and remove directories
- npm start
  - Used during development
  - Builds, watches, and serves with hot reloading
  - Automatically opens a browser window
- Make changes to the HTML in src/component/home/mod.rs
  - Or the CSS in public/template/stylesheet.css
- Note that the changes are updated in your browser as soon as you save

## Test Static Prerendering with Hydration

- npm test
  - Deletes the build and distribution directories to start clean
  - Makes the index.html page with the hydration code
  - Launches http-server to serve the HTML
  - Opens your browser to the home page

## Additional Run Script Commands

- npm run clean
  - Deletes the build and distribution directories to start clean
- npm run copy
  - Copies from the build to the distribution directory dist/
- npm run dist
  - Runs the clean, build, and copy scripts
  - Used to generate an SSG distribution in the dist/ directory
  - The dist/ files can be hosted on a Content Delivery Network (CDN)
- npm run format
  - Runs the "prettier" utility to format the generated files in dist/
  - Useful for analyzing or debugging the generated files
- npm run merge
  - Merges the static files in merge/ into dist/
- npm run serve
  - Starts the http-server in dist/
  - Opens the browser
- npm start
  - Described in a previous section
- npm test
  - Described in a previous section

## History

- 2022-08-21: Initial release
- 2024-12-30: Updated from Dioxus v0.4 to v0.6
- 2025-12-23: Updated from Dioxus v0.6 to v0.7
