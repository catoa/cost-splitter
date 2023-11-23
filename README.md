# splitter

Simple CLI tool to split itemized bills and receipts

## Installation

To install `splitter` run the following command:

`cargo install --path .`

Alternatively, you can clone this project and run it with `cargo run charge --names "Tony, Carl"`

## Usage

`splitter charge --names "Tony,Carl"`

This will trigger a prompt to enter the names and prices of the items that were purchased/ordered by each person. At the end you will receive a breakdown of what each individual owes.

```
Tony
==========
Ribeye: $20.00
Total Charges: $20.00
Percent of Subtotal (Total Charges / Subtotal): 33.33%
Fees owed: $13.33
Total Owed: $33.33

Carl
==========
Chicken: $40.00
Total Charges: $40.00
Percent of Subtotal (Total Charges / Subtotal): 66.67%
Fees owed: $26.67
Total Owed: $66.67
```
