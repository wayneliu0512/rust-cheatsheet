// mod basic;
// mod collection;
// mod error_handling;
// mod generic;
// mod trait_;
// mod lifetime;
// mod functional;
mod smart_pointer;

fn main() {
    // basic::primitive_type();
    // basic::compound_type();
    // basic::struct_();
    // basic::enum_();

    // collection::vector_();
    // collection::string_();
    // collection::hash_map_();

    // error_handling::demonstrate();

    // generic::function();
    // generic::struct_();
    // generic::multi_generic_types();

    // trait_::demonstrate();

    // lifetime::function();
    // lifetime::struct_();
    // lifetime::static_();
    // lifetime::generic_trait_bound_lifetime();

    // functional::closure();
    // functional::fn_trait();

    smart_pointer::deref();
    smart_pointer::deref_coercion();
    smart_pointer::drop();
    smart_pointer::rc();
    smart_pointer::ref_cell();
}
