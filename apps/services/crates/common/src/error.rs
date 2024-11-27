use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO {0}")]
    IO(#[from] std::io::Error),
}
