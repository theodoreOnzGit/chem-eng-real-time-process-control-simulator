use crate::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnFirstOrder;
use uom::si::f64::*;
use uom::ConstZero;
use uom::si::time::second;

/// a filtered derivative controller 
///
/// G(s) = s / (0.1 s + 1)
///
/// The form is identical to that of a first order transfer function 
/// with s = 0 as its only zero
///
/// Therefore, I'll just have this struct house a transfer function
///
///
pub struct FilteredDerivativeController{
    pub transfer_fn: TransferFnFirstOrder,
}

impl Default for FilteredDerivativeController {
    /// gives: 
    /// G(s) = s / (0.1 s + 1)
    fn default() -> Self {

        // G(s) = (a1 s + b1)/(a2 s + b2)
        //
        // a1 = 1 second 
        // b1 = 0 (ratio)
        // a2 = 0.1 second 
        // b2 = 0 (ratio)
        let b1 = Ratio::ZERO;
        let b2 = Ratio::ZERO;
        let a1 = Time::new::<second>(1.0);
        let a2 = Time::new::<second>(0.1);
        let transfer_fn = TransferFnFirstOrder::new(a1, b1, a2, b2).unwrap();

        return Self { transfer_fn };

    }
}

