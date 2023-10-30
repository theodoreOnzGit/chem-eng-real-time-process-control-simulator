#[macro_use]
extern crate approx;
mod examples;
fn main() {
    println!("library_demo");
    examples::second_order_demos::stable_second_order_simulation();
    examples::second_order_demos::no_zeroes_stable_underdamped_second_order_simulation();
    examples::second_order_demos::decaying_sine_stable_underdamped_second_order_simulation();
    examples::second_order_demos::demo_complex_stable_underdamped_second_order_simulation();
    examples::first_order_demos::stable_first_order_with_delay_simulation_no_zeroes();
    examples::first_order_demos::stable_first_order_with_delay_simulation_with_zeroes();
}

