mod basic;
mod collection;
mod error_handling;

fn main() {
    basic::primitive_type();
    basic::compound_type();
    basic::struct_();
    basic::enum_();

    collection::vector_();
    collection::string_();
    collection::hash_map_();

    error_handling::result();
}
