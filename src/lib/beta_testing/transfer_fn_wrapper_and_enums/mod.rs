/// generic enum for a Transfer Function
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum TransferFn {
    FirstOrderTransferFn,
    SecondOrderTransferFn(SecondOrder),
}

impl Default for TransferFn {
    fn default() -> TransferFn {
        todo!();
    }
}



pub mod generic_second_order;
pub use generic_second_order::SecondOrder;


