use crate::alpha_nightly::transfer_fn_wrapper_and_enums::{TransferFnFirstOrder, TransferFnTraits};
use csv::Writer;
use uom::si::f64::*;
use uom::ConstZero;
use uom::si::frequency::hertz;
use uom::si::ratio::ratio;
use uom::si::time::second;
use crate::alpha_nightly::errors::ChemEngProcessControlSimulatorError;

use super::Controller;

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
    /// current ramp function
    pub(crate) main_ramp_fn: RampResponse,
    /// vector of delayed ramp functions 
    pub(crate) delayed_ramp_fn_vectors: Vec<RampResponse>,
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
            // add to the delayed ramp response vector
            
            let ramp_fn_gain_gradient: Frequency = 
                self.controller_gain /  self.integral_time;
            let ramp_fn_start_time: Time = time_of_input + 
                self.delay;
            let ramp_fn_user_input: Ratio = user_input;
            let ramp_fn_current_time = time_of_input;

            let new_ramp_response = RampResponse::new(
                ramp_fn_gain_gradient, 
                ramp_fn_start_time, 
                ramp_fn_user_input, 
                ramp_fn_current_time)?;

            self.delayed_ramp_fn_vectors.push(new_ramp_response);

            // then we need to change the previous_timestep_input 
            // to the current input value 
            self.previous_timestep_input = user_input;

            // then we are done!
            
        }
        // clean up vector first 
        self.clear_ramp_response_vector(time_of_input);
        let summation_of_responses: Ratio = self.delayed_ramp_fn_vectors.
            iter_mut().map(
                |first_order_response|{
                    first_order_response.calculate_response(time_of_input)}
            ).sum();

        let main_ramp_fn_response = self.main_ramp_fn.calculate_response(
            time_of_input);

        let output = self.offset + summation_of_responses + 
            main_ramp_fn_response;

        return Ok(output);

    }


    fn spawn_writer(&mut self, name: String) -> Result<csv::Writer<std::fs::File>,
    ChemEngProcessControlSimulatorError> {
        let mut title_string: String = name;
        title_string += "_integral_controller.csv";
        let wtr = Writer::from_path(title_string)?;
        Ok(wtr)
    }

    fn csv_write_values(&mut self, 
        wtr: &mut csv::Writer<std::fs::File>,
        time: Time,
        input: Ratio,
        output: Ratio) -> Result<(), 
    ChemEngProcessControlSimulatorError> {
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

impl IntegralController {

    /// integral controller in the form: 
    ///
    /// G(s) = Kc / (tau_I s)
    pub fn new(controller_gain: Ratio,
        integral_time: Time) -> Result<Self,ChemEngProcessControlSimulatorError> {

        let main_ramp_fn: RampResponse = 
            RampResponse::new(
                Frequency::ZERO, 
                Time::ZERO, 
                Ratio::ZERO, 
                Time::ZERO)?;

        Ok(Self { controller_gain,
            integral_time,
            delay: Time::ZERO,
            previous_timestep_input: Ratio::ZERO,
            offset: Ratio::ZERO,
            main_ramp_fn,
            delayed_ramp_fn_vectors: vec![]
        })
    }

    /// for integral controllers, the algorithm is similar to the 
    /// first order decay type 
    /// 
    /// However, instead of waiting for the functions to decay out,
    ///
    /// we wait for about 1seconds after the ramp functions start
    /// after 1s, we take that response and use it to change 
    /// the current main ramp function
    ///
    /// so, after 1s, we consider the result "stabilised"
    fn clear_ramp_response_vector(&mut self, time_of_input: Time){


        let index_of_stabilised_result = self.delayed_ramp_fn_vectors.iter_mut().position(
            |ramp_fn_response| {
                ramp_fn_response.current_time = time_of_input;
                ramp_fn_response.is_started_for_1s()
            }
        );

        match index_of_stabilised_result {

            // if I found something at the index, remove it, 
            // repeatedly test it until nothing is left
            Some(index) => {
                // first get the current value of the ramp response 
                // and add it to the offset
                let mut ramp_fn_response = 
                    self.delayed_ramp_fn_vectors[index].clone();
                let current_value_of_ramp_response = 
                    ramp_fn_response.calculate_response(
                        time_of_input);
                self.offset += current_value_of_ramp_response;

                // second, I adjust the gradient of the main 
                // ramp response 

                let ramp_fn_prevailing_gradient = 
                    ramp_fn_response.gradient_gain
                    * ramp_fn_response.user_input;

                // update the main ramp function
                self.main_ramp_fn.gradient_gain += ramp_fn_prevailing_gradient;
                self.main_ramp_fn.user_input = ramp_fn_response.user_input;
                self.main_ramp_fn.start_time = time_of_input;
                self.main_ramp_fn.current_time = time_of_input;

                // then i remove the first order response from the 
                // index
                self.delayed_ramp_fn_vectors.remove(index);
            },

            // if no vectors reach steady state, exit
            // with no issue
            None => return,
        }

        // now, we have cleared the vector once, if there are other 
        // times we need to clear the vector, then we enter a loop

        let index_of_stabilised_result = self.delayed_ramp_fn_vectors.iter_mut().position(
            |ramp_fn_response| {
                ramp_fn_response.current_time = time_of_input;
                ramp_fn_response.is_started_for_1s()
            }
        );
        // check if steady state responses are present
        let mut stabilised_responses_present = 
            match index_of_stabilised_result {
                Some(_) => true,
                None => false,
            };
        
        if !stabilised_responses_present {
            return;
        } 

        // repeatedly clear the vector until no steady state responses 
        // are left
        while stabilised_responses_present {

            // check for index
            let index_of_steady_state_result = self.delayed_ramp_fn_vectors.iter_mut().position(
                |ramp_fn_response| {
                    ramp_fn_response.current_time = time_of_input;
                    ramp_fn_response.is_started_for_1s()
                }
            );

            stabilised_responses_present = match index_of_steady_state_result {
                Some(index) => {
                    // first get the current value of the ramp response 
                    // and add it to the offset
                    let mut ramp_fn_response = 
                        self.delayed_ramp_fn_vectors[index].clone();
                    let current_value_of_ramp_response = 
                        ramp_fn_response.calculate_response(
                            time_of_input);
                    self.offset += current_value_of_ramp_response;

                    // second, I adjust the gradient of the main 
                    // ramp response 

                    let ramp_fn_prevailing_gradient = 
                        ramp_fn_response.gradient_gain
                        * ramp_fn_response.user_input;
                    // update the main ramp function
                    self.main_ramp_fn.gradient_gain += ramp_fn_prevailing_gradient;
                    self.main_ramp_fn.user_input = ramp_fn_response.user_input;
                    self.main_ramp_fn.start_time = time_of_input;
                    self.main_ramp_fn.current_time = time_of_input;

                    // then i remove the first order response from the 
                    // index
                    self.delayed_ramp_fn_vectors.remove(index);


                    // return true value to while loop
                    true
                },
                // return false value to while loop
                None => false,
            };

        }
        return;

    }
}

/// Ramp response for integral controller 
/// able to take in a time varying input
///
/// Transfer function is G(s) = 1/s
#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub struct RampResponseRealTime {
    pub(crate) start_time: Time,
    pub(crate) current_time: Time,
    /// previous timestep input
    pub(crate) previous_timestep_input: Ratio,

    /// offset for ramp function calculations
    pub(crate) offset: Ratio,

    /// gradient gain 
    /// it is: Kc/tau_i 
    pub(crate) gradient_gain: Frequency,
    
}

impl Default for RampResponseRealTime {
    /// returns G(s) = 1/s
    fn default() -> Self {
        Self { start_time: Time::ZERO, 
            current_time: Time::ZERO, 
            previous_timestep_input: Ratio::ZERO, 
            offset: Ratio::ZERO,
            gradient_gain: Frequency::new::<hertz>(1.0),
        }
    }
}

impl RampResponseRealTime {

    pub fn new(integral_time: Time,
    controller_gain: Ratio) -> Result<Self,ChemEngProcessControlSimulatorError> {
        // we start with unit ramp
        let mut ramp_response = Self::default();

        ramp_response.gradient_gain = controller_gain/integral_time;

        return Ok(ramp_response);
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

        // if input changed, then we must change the gradient and 
        // the offset
        if input_changed {
            let k_c_over_tau_i: Frequency = self.gradient_gain;
            let a_i = user_input;

            let gradient_change: Frequency = a_i * k_c_over_tau_i;
            let offset_change: Ratio = -a_i*time_of_input *k_c_over_tau_i;

            self.offset += offset_change;
            self.gradient_gain += gradient_change;
        }

        // now calc based on the linear input: 

        let output = time_of_input * self.gradient_gain + self.offset;

        return Ok(output);
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
    pub fn calculate_response(&mut self, 
        simulation_time: Time)
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

    /// checks if ramp function is past its dead time
    /// for 1s or more
    pub fn is_started_for_1s(&self) -> bool {
            let time_elapsed = self.current_time - self.start_time;

            let turned_on_for_1s: bool = time_elapsed >= 
                Time::new::<second>(1.0);

            

            // if the current time is before start time, no response 
            // from this transfer function
            if !turned_on_for_1s {
                return false;
            }

            return true;
    }

}

impl Into<Controller> for IntegralController {
    fn into(self) -> Controller {
        Controller::IntegralStandalone(self)
    }
}
