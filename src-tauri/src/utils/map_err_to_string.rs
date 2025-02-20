pub trait MapErrToString<T> {
    fn map_err_to_string(self) -> std::result::Result<T, String>;
}

impl<T, E: ToString> MapErrToString<T> for Result<T, E> {
    fn map_err_to_string(self) -> std::result::Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
