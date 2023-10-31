/// Copyright [2023] [Theodore Kay Chen Ong, Professor Per F. Peterson,
/// Thermal Hydraulics Lab, Repository Contributors and 
/// Singapore Nuclear Research and Safety Initiative (SNRSI)]
/// 
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
pub mod prelude;
pub(crate) mod stable_transfer_functions;
pub(crate) mod controllers;
pub mod errors;
pub mod transfer_fn_wrapper_and_enums;


use uom::si::{Quantity, ISQ, SI};
use uom::typenum::*;
pub(crate) type TimeSquared = 
Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

// Time squared unit for use in second order functions

#[test]
pub fn timesq_test(){
    // this just tests the time squared unit
    use uom::si::{time::second, f64::Time};

    let a = Time::new::<second>(1.0);
    let a_sq: TimeSquared = a*a;
    assert_eq!(a*a, a_sq);
}

