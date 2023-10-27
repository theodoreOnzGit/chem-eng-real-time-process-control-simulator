use uom::si::{f64::*, Quantity, Dimension, ISQ, SI};
use uom::typenum::*;

use crate::beta_testing::TimeSquared;
use crate::beta_testing::stable_transfer_functions::second_order_transfer_fn::SecondOrderStableTransferFnNoZeroes;

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum SecondOrder {
    Stable,
    Unstable,
    Undamped,
}

impl Default for SecondOrder {
    fn default() -> Self {
        todo!()
    }
}

impl SecondOrder {


    /// generic constructor based on polynomials
    /// This is in the form 
    ///
    /// G(s) = 
    ///
    /// a1 s^2 + b1 s + c1
    /// ------------------
    /// a2 s^2 + b2 s + c2
    ///
    ///
    /// Unfortunately, uom does not have a time^2 unit yet,
    /// So I'm using the manual way, to define a new unit 
    /// using uom
    /// https://github.com/iliekturtles/uom/issues/174
    /// This is how one writes a time squared unit
    /// Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>
    /// 
    pub fn new(a1: TimeSquared,
    b1: Time,
    c1: Ratio,
    a2: TimeSquared,
    b2: Time,
    c2: Ratio) -> Self {

        // process time 
        let tau_p: Time = (a2/c2).sqrt();

        // damping factor
        let zeta: Ratio = 0.5 * b2 / (a2*c2).sqrt();

        // process_gain 
        // I assume units of c1 are dimensionless
        let k_p: Ratio = c1/c2;

        // decay constant 
        let lambda = 0.5 *b2/a2;

        // angular frequency for decaying sinusoids
        let omega: Frequency = (c2/a2 - 0.25*b2*b2/a2/a2).sqrt();

        todo!()

    }

    // underdamped stable systems
    #[inline]
    fn new_underdamped_stable_system(){

        // underdamped systems will contain two decaying_sinusoid
        // types and one SecondOrderStableTransferFunction Type
        // 
        let mut second_order_stable_transfer_fn_no_zeroes: 
        SecondOrderStableTransferFnNoZeroes;



    }

}


