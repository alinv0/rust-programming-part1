mod demo_nested_functions;
mod demo_closures;
mod demo_closures_inferred_types;
mod demo_closures_capture_reference;
mod demo_closures_capture_value;
mod demo_closures_iteration;

fn main() {
    demo_nested_functions::do_it();
    demo_closures::do_it();
    demo_closures_inferred_types::do_it();
    demo_closures_capture_reference::do_it();
    demo_closures_capture_value::do_it();
    demo_closures_iteration::do_it();
}
