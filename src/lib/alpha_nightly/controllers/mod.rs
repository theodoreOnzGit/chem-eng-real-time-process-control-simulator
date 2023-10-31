use csv::Writer;
use uom::si::f64::*;

pub use self::integral_controller::IntegralController;
pub use self::proportional_controller::ProportionalController;
pub use self::filtered_derivative_controller::FilteredDerivativeController;
pub(crate) mod proportional_controller;
pub(crate) mod integral_controller;
pub(crate) mod filtered_derivative_controller;

/// generic enum for a Transfer Function
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Controller {
    PIDFiltered(ProportionalController,IntegralController,FilteredDerivativeController),
    PI(ProportionalController,IntegralController),
    P(ProportionalController),
    IntegralStandalone(IntegralController),
}
