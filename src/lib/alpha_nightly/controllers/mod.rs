use csv::Writer;
use uom::si::f64::*;

use self::filtered_derivative_controller::FilteredDerivativeController;
pub(crate) mod proportional_controller;
pub(crate) mod integral_controller;
pub(crate) mod filtered_derivative_controller;

/// generic enum for a Transfer Function
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Controller {
    PIDFiltered(FilteredDerivativeController),
    PI,
    P,
}
