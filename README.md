# splitter

Simple CLI tool to split itemized bills and receipts with comprehensive test coverage and modern Rust patterns.

## Features

- **Manual Charge Entry**: Interactive CLI for entering items and costs per person
- **OCR Receipt Processing**: AWS Textract integration for automated receipt parsing
- **Fair Cost Splitting**: Proportional fee distribution based on individual charges
- **Comprehensive Testing**: Unit, integration, and property-based tests
- **Modern Rust**: Idiomatic patterns with proper error handling and dependency injection

## Installation

To install `splitter` run the following command:

`cargo install --path .`

Alternatively, you can clone this project and run it with `cargo run charge --names "Tony, Carl"`

## Usage

### Manual Charge Entry

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

### OCR Receipt Processing

`splitter ocr --receipt-path /path/to/receipt.jpg`

This will use AWS Textract to parse the receipt and allow you to assign items to people.

## Development

### Running Tests

The project has comprehensive test coverage including:

- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test full workflows end-to-end
- **Property-Based Tests**: Automated testing with randomized inputs

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test charges_tests
cargo test --test integration_tests
cargo test --test property_tests

# Run with verbose output
cargo test -- --nocapture
```

### Code Quality

The codebase follows modern Rust best practices:

- **Error Handling**: Custom error types with `thiserror`
- **Dependency Injection**: Testable I/O operations with trait abstractions
- **Type Safety**: Strong typing throughout with proper `Result` handling
- **Documentation**: Comprehensive inline documentation

### Architecture

- `src/io.rs`: I/O abstraction traits for testability
- `src/errors.rs`: Comprehensive error handling
- `src/charges.rs`: Core charge processing logic
- `src/billing.rs`: Bill calculation and display
- `src/actions.rs`: User input parsing and actions
- `src/ocr.rs`: AWS Textract integration
- `tests/`: Comprehensive test suites

### Contributing

When contributing:

1. Ensure all tests pass: `cargo test`
2. Format code: `cargo fmt`
3. Run lints: `cargo clippy`
4. Add tests for new functionality
5. Update documentation as needed
