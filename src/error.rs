#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error of io: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error of isahc: {0}")]
    Isahc(#[from] isahc::Error),

    #[error("Error of isahc http: {0}")]
    IsahcHttp(#[from] isahc::http::Error),

    #[error("Error of indicatif iemplate: {0}")]
    IndicatifTemplate(#[from] indicatif::style::TemplateError),

    #[error("Error: {0}")]
    Any(#[from] color_eyre::Report),
}

pub type Result<T> = std::prelude::rust_2024::Result<T, crate::Error>;
