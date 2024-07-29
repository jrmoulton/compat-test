pub trait SerializeCompat<PT> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}
