#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error of io: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error of reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Error: {0}")]
    Any(#[from] color_eyre::Report),
}

pub type Result<T> = std::prelude::rust_2024::Result<T, crate::Error>;

// pub trait OptionExt<T> {
//     fn ok_or_any(self, any: color_eyre::Report) -> crate::Result<T>;
// }

// impl<T> OptionExt<T> for Option<T> {
//     fn ok_or_any(self, any: color_eyre::Report) -> crate::Result<T> {
//         match self {
//             Some(t) => Ok(t),
//             None => Err(any.into()),
//         }
//     }
// }
