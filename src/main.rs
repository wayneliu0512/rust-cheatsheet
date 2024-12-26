mod basic;
mod collection;
mod error_handling;
mod generic;
mod trait_;
mod lifetime;

fn main() {
    basic::primitive_type();
    basic::compound_type();
    basic::struct_();
    basic::enum_();

    collection::vector_();
    collection::string_();
    collection::hash_map_();

    error_handling::demonstrate();

    generic::function();
    generic::struct_();
    generic::multi_generic_types();

    trait_::demonstrate();

    lifetime::function();
    lifetime::struct_();
}
