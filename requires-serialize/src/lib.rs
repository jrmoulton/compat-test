pub fn requires_serialized<S, T: serialize::SerializeCompat<S>>(val: T) {
    let mut serializer = serde_json::Serializer::new(Vec::new());
    val.serialize(&mut serializer).unwrap();
    let serialized_str = String::from_utf8(serializer.into_inner()).unwrap();
    println!("{}", serialized_str);
}
