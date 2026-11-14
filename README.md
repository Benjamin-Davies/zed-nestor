# Zed Nestor

Zed extension for the Nestor C LSP.

## Usage

You will need [cargo](https://rustup.rs/) and Zed installed.

* Install [Nestor](https://github.com/Benjamin-Davies/nestor), e.g. `cargo install --git https://github.com/Benjamin-Davies/nestor`.
* Clone this repo.
* In Zed, open the command palette, select "zed: install dev extension", and select the repo you just cloned.
* Configure Zed to use Nestor for C files:
  ```json
  {
    "languages": {
      "C": {
        "language_servers": ["nestor"]
      }
    }
  }
  ```
