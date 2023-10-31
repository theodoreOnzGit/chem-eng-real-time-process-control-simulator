use crate::alpha_nightly::transfer_fn_wrapper_and_enums::{TransferFnFirstOrder, TransferFnTraits};
use uom::si::f64::*;
use uom::ConstZero;
use uom::si::frequency::hertz;
use uom::si::ratio::ratio;
use uom::si::time::second;
use crate::alpha_nightly::errors::ChemEngProcessControlSimulatorError;

/// Integral controller with transfer function
///
/// G(s) = K_c / (tau_I s)
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub struct IntegralController{
    pub(crate) controller_gain: Ratio,
    /// also known as reset time
    pub(crate) integral_time: Time,
    /// also known as dead time
    pub(crate) delay: Time,
    /// previous timestep input
    pub(crate) previous_timestep_input: Ratio,

    /// offset for ramp function calculations
    pub(crate) offset: Ratio,
    /// current ramp function gradient
    pub(crate) ramp_function_gradient: Ratio,
}

impl Default for IntegralController {
    fn default() -> Self {
        todo!()
    }
}

impl TransferFnTraits for IntegralController {
    fn set_dead_time(&mut self, dead_time: Time) {
        self.delay = dead_time;
    }

    fn set_user_input_and_calc(&mut self, 
        user_input: Ratio,
        time_of_input: Time) -> Result<Ratio, 
    ChemEngProcessControlSimulatorError> {
        // check if input is equal to current input 

        // case where input is not the same to 9 decimal places

        let input_changed: bool = 
            (user_input.get::<ratio>() * 1e9).round() 
            - (self.previous_timestep_input.clone()
                .get::<ratio>()*1e9).round() != 0.0 ;


        if input_changed {
            // need to adjust offset and gradient immediately



            // then we need to change the previous_timestep_input 
            // to the current input value 
            self.previous_timestep_input = user_input;

            // then we are done!
            
        }

        todo!()

    }

    fn spawn_writer(&mut self, name: String) -> Result<csv::Writer<std::fs::File>,
    ChemEngProcessControlSimulatorError> {
        todo!()
    }

    fn csv_write_values(&mut self, 
        wtr: &mut csv::Writer<std::fs::File>,
        time: Time,
        input: Ratio,
        output: Ratio) -> Result<(), 
    ChemEngProcessControlSimulatorError> {
        todo!()
    }
}

/// Ramp response for integral controller, 
/// This is because the integral of a step function is 
/// a ramp response
///
/// Allows for a user defined start time where the ramp 
/// response switches on
///
///
/// The response is:
///
/// y(t) = u (t - t_start) * a1 * K * (t - t_start)
///
#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub struct RampResponse {
    pub(crate) gradient_gain: Frequency,
    pub(crate) start_time: Time,
    pub(crate) user_input: Ratio,
    pub(crate) current_time: Time,
}

impl Default for RampResponse {
    fn default() -> Self {
        RampResponse { 
            gradient_gain: Frequency::new::<hertz>(1.0), 
            start_time: Time::ZERO, 
            user_input: Ratio::new::<ratio>(1.0), 
            current_time: Time::ZERO
        }
    }
}

impl RampResponse {

    /// constructor
    pub fn new(
        gradient_gain: Frequency,
        start_time: Time,
        user_input: Ratio,
        current_time:Time) -> Result<Self, ChemEngProcessControlSimulatorError>{
        
        Ok(Self { 
            gradient_gain, 
            start_time, 
            user_input, 
            current_time 
        })
    }

    /// calculates the current value of the ramp response 
    pub fn calculate_response(&mut self, simulation_time:Time)
        -> Ratio {

            // get the current time (t - t0)
            self.current_time = simulation_time;
            let time_elapsed = self.current_time - self.start_time;

            let heaviside_on: bool = self.current_time >= self.start_time;


            // if the current time is before start time, no response 
            // from this transfer function
            if !heaviside_on {
                return Ratio::ZERO;
            }

            // or else just calculate the response 

            let response: Ratio = self.user_input 
                * self.gradient_gain 
                * time_elapsed;
            return response;

        }


}
