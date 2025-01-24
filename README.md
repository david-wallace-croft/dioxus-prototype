# Dioxus Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-prototype/blob/main/LICENSE.txt

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
  - TODO: Update this tutorial from Dioxus v0.4 to v0.6

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

- npm install
- npm test

## npm scripts

- npm start
  - Used during development
  - Builds, watches, and serves with hot reloading
  - Automatically opens a browser window
- npm run clean
  - Deletes the build output and distribution directories
- npm run build
  - Builds a release version with static site generation (SSG)
- npm run merge
  - Makes the distribution directory dist/
  - Merges the release build into dist/
  - Merges the generated SSG files into dist/
  - Merges the static files in merge/ into dist/
- npm run dist
  - Runs the clean, build, and merge scripts
  - Used to generate an SSG distribution in the dist/ directory
  - The dist/ files can be hosted on a Content Delivery Network (CDN)
- npm run serve
  - Serves the files in the distribution directory dist/
  - Automatically opens a browser window
- npm test
  - Runs the dist and serve scripts
  - Used to test the SSG distribution prior to hosting on a CDN
- npm run format
  - Runs the utility prettier to format the generated SSG files

## TODO

- Restore the prettier configuration
- Bump the version number

## History

- 2022-08-21: Initial release
- 2024-12-30: Updated from Dioxus v0.4 to v0.6
