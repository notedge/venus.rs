#[derive(Debug, Copy, Clone)]
pub enum VistaError {
    UnknownError
}

pub type Result<T> = std::result::Result<T, VistaError>;
