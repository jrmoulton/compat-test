use serialize::SerializeCompat;
use type_one::TypeOne;

pub struct _CompatType;

impl SerializeCompat<_CompatType> for TypeOne {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("TypeOne")
    }
}
