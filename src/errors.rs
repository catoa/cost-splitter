use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum SplitterError {
    #[error("Failed to parse input action: {0}")]
    ParseInputAction(String),

    #[error("Failed to parse charge from input: {0}")]
    ParseCharge(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("AWS Textract error: {0}")]
    AwsTextract(String),

    #[error("Invalid bill total: {total}, must be >= subtotal: {subtotal}")]
    InvalidBillTotal { total: f64, subtotal: f64 },

    #[error("No charges found for person: {0}")]
    NoChargesFound(String),

    #[error("Invalid charge index: {0}")]
    InvalidChargeIndex(usize),

    #[error("Unassigned charges remaining")]
    UnassignedCharges,
}

pub type Result<T> = std::result::Result<T, SplitterError>;

impl From<aws_sdk_textract::error::SdkError<aws_sdk_textract::operation::analyze_expense::AnalyzeExpenseError>> for SplitterError {
    fn from(err: aws_sdk_textract::error::SdkError<aws_sdk_textract::operation::analyze_expense::AnalyzeExpenseError>) -> Self {
        SplitterError::AwsTextract(format!("{}", err))
    }
}
