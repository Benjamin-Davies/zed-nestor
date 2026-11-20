# Zed Nestor

Zed extension for the Nestor C LSP.

## Usage

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
