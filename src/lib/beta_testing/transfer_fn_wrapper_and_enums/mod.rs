use uom::si::f64::*;
/// generic enum for a Transfer Function
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum TransferFn {
    FirstOrder,
    SecondOrder(TransferFnSecondOrder),
}

impl Default for TransferFn {
    fn default() -> TransferFn {
        todo!();
    }
}



pub trait TransferFnTraits {
    fn set_dead_time(&mut self, dead_time: Time);
    fn csv_plot(&self);
    fn set_user_input_and_calc(&mut self, 
        user_input: Ratio,
        time_of_input: Time) -> Ratio;

}


pub mod generic_second_order;
pub use generic_second_order::TransferFnSecondOrder;


