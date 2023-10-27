pub(crate) mod prelude;
pub(crate) mod stable_transfer_functions;
pub(crate) mod controllers;
pub mod errors;
pub mod transfer_fn_wrapper_and_enums;


use uom::si::{Quantity, ISQ, SI};
use uom::typenum::*;
pub(crate) type TimeSquared = 
Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

// Time squared unit for use in second order functions

#[cfg(test)]
pub fn timesq_test (){
    // this just tests the time squared unit
    use uom::si::{time::second, f64::Time};

    let a = Time::new::<second>(1.0);
    let a_sq: TimeSquared = a*a;
    assert_eq!(a*a, a_sq);
}

