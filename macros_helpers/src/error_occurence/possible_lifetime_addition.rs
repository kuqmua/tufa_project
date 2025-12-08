pub fn possible_lifetime_addition(lifetime: String, lifetimes_for_serialize_deserialize: &mut Vec<String>) {
    if !lifetimes_for_serialize_deserialize.contains(&lifetime) {
        lifetimes_for_serialize_deserialize.push(lifetime);
    }
}
