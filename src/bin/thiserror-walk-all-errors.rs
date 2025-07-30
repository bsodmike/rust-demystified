use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum ValidateOnePaperDocumentErrorKind {
    #[error("another error")]
    AnotherError,
    #[error("you do not have the permissions to upload this document")]
    Forbidden,
}

#[derive(Debug, thiserror::Error)]
#[error("{document_kind}: {error_kind}")]
pub struct ValidateOnePaperDocumentError {
    document_kind: PaperDocuments,
    error_kind: ValidateOnePaperDocumentErrorKind,
}

#[derive(Debug)]
pub struct ValidatePaperDocumentsError {
    errors: Vec<ValidateOnePaperDocumentError>,
}

impl Default for ValidatePaperDocumentsError {
    fn default() -> Self {
        Self { errors: Vec::new() }
    }
}

#[derive(Debug)]
pub enum PaperDocuments {
    Office,
    Ledger,
}

impl Display for PaperDocuments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaperDocuments::Office => f.write_str("Office"),
            PaperDocuments::Ledger => f.write_str("Ledger"),
        }
    }
}

async fn validate_office_documents<'a, I: Iterator<Item = &'a (i32, PaperDocuments)>>(
    documents: I,
) -> Result<(), ValidatePaperDocumentsError> {
    let mut validation = ValidatePaperDocumentsError::default();

    for (document, kind) in documents {
        if *document == 1 {
            validation.errors.push(ValidateOnePaperDocumentError {
                document_kind: PaperDocuments::Office,
                error_kind: ValidateOnePaperDocumentErrorKind::Forbidden,
            });
        };

        if *document == 2 {
            validation.errors.push(ValidateOnePaperDocumentError {
                document_kind: PaperDocuments::Ledger,
                error_kind: ValidateOnePaperDocumentErrorKind::AnotherError,
            });
        };

        continue;
    }

    if !validation.errors.is_empty() {
        return Err(validation);
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("custom error: {0}")]
    ErrorMessage(String),
}

impl From<ValidatePaperDocumentsError> for CustomError {
    fn from(value: ValidatePaperDocumentsError) -> Self {
        let all_errors = value
            .errors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        Self::ErrorMessage(format!("paper documents validation error: {all_errors}"))
    }
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let docs = vec![
        (0, PaperDocuments::Office),
        (1, PaperDocuments::Ledger),
        (2, PaperDocuments::Office),
    ];

    let r = validate_office_documents(docs.iter()).await?;

    Ok(())
}
