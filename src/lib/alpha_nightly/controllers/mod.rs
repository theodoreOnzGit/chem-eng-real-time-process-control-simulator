
use csv::Writer;
use uom::si::ratio::ratio;
use uom::si::time::second;

pub use self::integral_controller::IntegralController;
pub use self::proportional_controller::ProportionalController;
pub use self::filtered_derivative_controller::FilteredDerivativeController;

use super::transfer_fn_wrapper_and_enums::TransferFnTraits;
pub(crate) mod proportional_controller;
pub mod integral_controller;
pub(crate) mod filtered_derivative_controller;

/// generic enum for a Transfer Function
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Controller {
    PIDFiltered(ProportionalController,IntegralController,FilteredDerivativeController),
    PI(ProportionalController,IntegralController),
    P(ProportionalController),
    IntegralStandalone(IntegralController),
}

impl TransferFnTraits for Controller {
    fn set_dead_time(&mut self, dead_time: uom::si::f64::Time) {
        match self {
            Controller::PIDFiltered(proportional_controller,
                integral_controller, filtered_derivative_controller) => {
                    proportional_controller.set_dead_time(dead_time);
                    integral_controller.set_dead_time(dead_time);
                    filtered_derivative_controller.set_dead_time(dead_time)
                },
            Controller::PI(p_controller, integral_controller) => {
                p_controller.set_dead_time(dead_time);
                integral_controller.set_dead_time(dead_time)
            },
            Controller::P(ctrl) => {
                ctrl.set_dead_time(dead_time)
            },
            Controller::IntegralStandalone(ctrl) => {
                ctrl.set_dead_time(dead_time)
            },
        }
    }

    fn set_user_input_and_calc(&mut self, 
        user_input: uom::si::f64::Ratio,
        time_of_input: uom::si::f64::Time) -> Result<uom::si::f64::Ratio, 
    super::errors::ChemEngProcessControlSimulatorError> {
        match self {
            Controller::PIDFiltered(proportional_controller,
                integral_controller, filtered_derivative_controller) => {
                    let p_output = 
                    proportional_controller.set_user_input_and_calc(user_input, time_of_input)?;
                    let i_output = 
                    integral_controller.set_user_input_and_calc(user_input, time_of_input)?;
                    let d_output = 
                    filtered_derivative_controller.set_user_input_and_calc(user_input, time_of_input)?;

                    return Ok(p_output + i_output + d_output);
                },
            Controller::PI(proportional_controller, integral_controller) => {
                let p_output = 
                proportional_controller.set_user_input_and_calc(user_input, time_of_input)?;
                let i_output = 
                integral_controller.set_user_input_and_calc(user_input, time_of_input)?;
                return Ok(p_output + i_output);
            },
            Controller::P(ctrl) => {
                ctrl.set_user_input_and_calc(user_input, time_of_input)
            },
            Controller::IntegralStandalone(ctrl) => {
                ctrl.set_user_input_and_calc(user_input, time_of_input)
            },
        }
    }

    fn spawn_writer(&mut self, name: String) -> Result<csv::Writer<std::fs::File>,
    super::errors::ChemEngProcessControlSimulatorError> {
        let mut title_string: String = name;
        match self {
            Controller::PIDFiltered(_, _, _) => {
                    title_string += "_PID_controller.csv"

                },
            Controller::PI(_, _) => {
                title_string += "_PI_controller.csv"
            },
            Controller::P(ctrl) => {
                return ctrl.spawn_writer(title_string);
            },
            Controller::IntegralStandalone(ctrl) => {
                return ctrl.spawn_writer(title_string);
            },
        }
        let wtr = Writer::from_path(title_string)?;
        Ok(wtr)
    }

    fn csv_write_values(&mut self, 
        wtr: &mut csv::Writer<std::fs::File>,
        time: uom::si::f64::Time,
        input: uom::si::f64::Ratio,
        output: uom::si::f64::Ratio) -> Result<(), 
    super::errors::ChemEngProcessControlSimulatorError> {
        let current_time_string = time.get::<second>().to_string();
        let input_string = input.get::<ratio>().to_string();
        let output_string = output.get::<ratio>().to_string();

        wtr.write_record(&[current_time_string,
            input_string,
            output_string])?;

        wtr.flush().unwrap();
        Ok(())
    }
}
