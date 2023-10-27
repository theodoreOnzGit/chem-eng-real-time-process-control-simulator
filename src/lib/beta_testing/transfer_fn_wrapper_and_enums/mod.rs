use csv::Writer;
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

impl TransferFnTraits for TransferFn {
    fn set_dead_time(&mut self, dead_time: Time) {
        match self {
            TransferFn::FirstOrder => todo!(),
            TransferFn::SecondOrder(second_order) => {
                second_order.set_dead_time(dead_time)
            },
        }
    }


    fn set_user_input_and_calc(&mut self, 
        user_input: Ratio,
        time_of_input: Time) -> 
    Result<Ratio, ChemEngProcessControlSimulatorError> {
        match self {
            TransferFn::FirstOrder => todo!(),
            TransferFn::SecondOrder(second_order) => {
                second_order.set_user_input_and_calc(user_input, time_of_input)
            },
        }
    }

    fn spawn_writer(&mut self, name: String) -> Result<Writer<std::fs::File>,
    ChemEngProcessControlSimulatorError>{
        match self {
            TransferFn::FirstOrder => todo!(),
            TransferFn::SecondOrder(second_order) => {
                second_order.spawn_writer(name)
            },
        }
    }


}



pub trait TransferFnTraits {
    fn set_dead_time(&mut self, dead_time: Time);
    fn set_user_input_and_calc(&mut self, 
        user_input: Ratio,
        time_of_input: Time) -> Result<Ratio, 
    ChemEngProcessControlSimulatorError>;

    fn spawn_writer(&mut self, name: String) -> Result<Writer<std::fs::File>,
    ChemEngProcessControlSimulatorError>;

}


pub mod generic_second_order;
pub use generic_second_order::TransferFnSecondOrder;

use super::errors::ChemEngProcessControlSimulatorError;


