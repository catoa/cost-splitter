# splitter

Simple CLI tool to split itemized bills and receipts

## Installation

To install `splitter` run the following command:

`cargo install --path .`

Alternatively, you can clone this project and run it with `cargo run charge --names "Tony, Carl"`

## Usage

`splitter charge --names "Tony,Carl"`

This will trigger a prompt to enter the names and prices of the items that were purchased/ordered by each person. At the end you will receive a breakdown of which items

To-Dos

- Add docs of example usage
- Add Dockerfile to run as executable
