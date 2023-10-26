use uom::si::{f64::*, time::second};

/// second order system with transfer function 
/// in the form 
///
/// K_p / ( tau^2 s^2 + 2 * tau * zeta s + 1)
///
/// tau is process time 
/// zeta is damping factor 
/// K_p is process gain (dimensionless, be careful)
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub struct SecondOrderStableTransferFn {
    process_gain: f64,
    process_time: Time,
    previous_timestep_input: f64,
    damping_factor: f64,
    /// previous timestep output
    offset: f64,
    /// delay
    delay: Time,

    /// vector of first order responses 
    response_vec: Vec<SecondOrderStableStepResponse>,
}

impl Default for SecondOrderStableTransferFn {
    /// default is: 
    ///
    /// 1 / (s^2 + 2s + 1)
    /// where process time is 1 second
    /// the damping factor is 1.0 which makes it a critically 
    /// damped system
    ///
    /// with initial user input of 0.0 
    /// and initial user value of 0.0
    fn default() -> Self {
        SecondOrderStableTransferFn { 
            process_gain: 1.0, 
            process_time: Time::new::<second>(1.0), 
            previous_timestep_input: 0.0, 
            offset: 0.0, 
            delay: Time::new::<second>(0.0), 
            response_vec: vec![],
            damping_factor: 1.0,
        }
    }
}

impl SecondOrderStableTransferFn {

    /// constructors 
    pub fn new(process_gain: f64,
        process_time: Time,
        damping_factor: f64,
        initial_input: f64,
        initial_value: f64,
        delay: Time,) -> Self {

        // if damping factor is less than or equal 
        // 0, should throw an error 
        // or panic (i will use errors maybe later?)

        if damping_factor <= 0.0 {
            todo!("damping factor needs to be more than 0.0, \n 
                also need to implement Result enum")
        }

        SecondOrderStableTransferFn { 
            process_gain, 
            process_time, 
            previous_timestep_input: initial_input, 
            offset: initial_value, 
            delay, 
            response_vec: vec![],
            damping_factor,
        }
    }

    /// second order filter 
    /// the system will be critically damped
    pub fn new_filter(process_time: Time,
        initial_input: f64,
        initial_value: f64) -> Self {
        SecondOrderStableTransferFn { 
            process_gain: 1.0, 
            process_time, 
            previous_timestep_input: initial_input, 
            offset: initial_value, 
            delay: Time::new::<second>(0.0), 
            response_vec: vec![],
            damping_factor: 0.5,
        }
    }

    /// sets the user input to some value
    pub fn set_user_input_and_calc_output(&mut self, 
        current_time: Time,
        current_input: f64) 
    -> f64 {
        // check if input is equal to current input 

        // case where input is not the same to 9 decimal places

        let input_changed: bool = 
            (current_input * 1e9).round() 
            - (self.previous_timestep_input.clone()*1e9).round() != 0.0 ;

        if input_changed {
            // need to add a response to the vector

            let process_gain = self.process_gain;
            let process_time = self.process_time;
            let user_input = current_input - self.previous_timestep_input;
            // the time where the first order response kicks in
            let start_time = current_time + self.delay;
            let damping_factor = self.damping_factor;

            // make a new response
            let new_response = SecondOrderStableStepResponse::new(
                process_gain,
                process_time,
                damping_factor,
                start_time,
                user_input,
                current_time
            );

            // add response to the vector
            self.response_vec.push(new_response);

            // then we need to change the previous_timestep_input 
            // to the current input value 
            self.previous_timestep_input = current_input;

            // then we are done!
            
        }

        // clean up the vector first
        self.clear_second_order_response_vector();

        // need to calculate using the list of 
        // first order response vectors as per normal
        //
        // So we are summing this up
        // O(t) = summing:: u2(t - t2) * b [1-exp(-a * [t-t2])] 
        // + offset
        // first we add the offset

        let summation_of_responses: f64 = self.response_vec.
            iter_mut().map(
                |second_order_response|{
                    second_order_response.calculate_response(current_time)}
            ).sum();

        let output = self.offset + summation_of_responses;

        return output;

    }

    /// clears the item if they have reached steady state
    fn clear_second_order_response_vector(&mut self){

        let index_of_steady_state_result = self.response_vec.iter().position(
            |first_order_response| {
                first_order_response.is_steady_state()
            }
        );

        match index_of_steady_state_result {

            // if I found something at the index, remove it, 
            // repeatedly test it until nothing is left
            Some(index) => {
                // first get the steady state value and add it to the 
                // offset
                let first_order_response = self.response_vec[index].clone();
                let steady_state_value_of_response = 
                    first_order_response.steady_state_value();
                self.offset += steady_state_value_of_response;

                // then i remove the first order response from the 
                // index
                self.response_vec.remove(index);
            },

            // if no vectors reach steady state, exit
            // with no issue
            None => return,
        }

        // now, we have cleared the vector once, if there are other 
        // times we need to clear the vector, then we enter a loop

        let index_of_steady_state_result = self.response_vec.iter().position(
            |first_order_response| {
                first_order_response.is_steady_state()
            }
        );
        // check if steady state responses are present
        let mut steady_state_responses_present = 
            match index_of_steady_state_result {
                Some(_) => true,
                None => false,
            };
        
        if !steady_state_responses_present {
            return;
        } 

        // repeatedly clear the vector until no steady state responses 
        // are left
        while steady_state_responses_present {

            // check for index
            let index_of_steady_state_result = self.response_vec.iter().position(
                |first_order_response| {
                    first_order_response.is_steady_state()
                }
            );

            steady_state_responses_present = match index_of_steady_state_result {
                Some(index) => {
                    // first get the steady state value and add it to the 
                    // offset
                    let first_order_response = self.response_vec[index].clone();
                    let steady_state_value_of_response = 
                        first_order_response.steady_state_value();
                    self.offset += steady_state_value_of_response;

                    // then i remove the first order response from the 
                    // index
                    self.response_vec.remove(index);

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




/// second order response struct, 
/// will help to caluclate
/// step responses for underdamped, crtically damped and 
/// overdamped stable systems
#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub struct SecondOrderStableStepResponse {
    process_gain: f64,
    process_time: Time,
    start_time: Time,
    user_input: f64,
    current_time: Time,
    damping_factor: f64,
}

impl Default for SecondOrderStableStepResponse {
    /// default is a critically damped system with 
    /// process time 1s, 
    /// process gain 1.0 (dimensionless)
    fn default() -> Self {
        SecondOrderStableStepResponse { 
            process_gain: 1.0, 
            process_time: Time::new::<second>(1.0), 
            start_time: Time::new::<second>(0.0), 
            user_input: 1.0, 
            current_time: Time::new::<second>(0.0),
            damping_factor: 1.0,
        }
    }
}


impl SecondOrderStableStepResponse {

    /// constructor 
    pub fn new(
        process_gain: f64,
        process_time: Time,
        damping_factor: f64,
        start_time: Time,
        user_input: f64,
        current_time: Time,) -> Self {

        // if damping factor is less than or equal 
        // 0, should throw an error 
        // or panic (i will use errors maybe later?)

        if damping_factor <= 0.0 {
            todo!("damping factor needs to be more than 0.0, \n 
                also need to implement Result enum")
        }
        SecondOrderStableStepResponse { 
            process_gain, 
            process_time, 
            start_time, 
            user_input, 
            current_time,
            damping_factor,
        }
    }

    /// checks if the transfer function has more or less reached 
    /// steady state,
    ///
    /// I consider this where the time elapsed is 23 times 
    /// the process_time
    ///
    /// this is because 23 * exp(-23) is about 2e-9, it is tiny...
    /// this is because we need to consider the exponential of 
    /// x exp(-x) for critically damped systems
    pub fn is_steady_state(&self) -> bool {
        let time_elapsed = self.current_time - self.start_time;

        let time_ratio: f64 = time_elapsed.value/self.process_time.value;

        let damping_factor = self.damping_factor;
        // no unstable or undamped responses allowed
        if damping_factor <= 0.0 {
            todo!("damping factor needs to be more than 0.0, \n 
                also need to implement Result enum")
        }

        if damping_factor < 1.0 {
            // case 1: underdamped systems
            // (zeta * t/tau_p) > 20.0

            let underdamped_time_ratio = damping_factor * time_ratio;

            if underdamped_time_ratio > 20.0 {
                return true;
            }

        } else if damping_factor == 1.0 {

            // case 2: critically damped system
            if time_ratio > 23.0 {
                return true;
            }

        } else {
            // case 3: overdamped system
            let sqrt_zeta_sq_minus_one: f64 = 
                (damping_factor.powf(2.0) - 1.0).sqrt();
            let zeta = damping_factor;

            let overdamped_time_ratio_one = 
                (zeta - sqrt_zeta_sq_minus_one) * time_ratio;

            let overdamped_time_ratio_two = 
                (zeta + sqrt_zeta_sq_minus_one) * time_ratio;

            let overdamped_mode_one_steady_state: bool = 
                overdamped_time_ratio_one.abs() > 20.0;
            let overdamped_mode_two_steady_state: bool = 
                overdamped_time_ratio_two.abs() > 20.0;

            if overdamped_mode_two_steady_state && 
                overdamped_mode_one_steady_state {
                    return true;
            }

        }

        



        return false;
    }


    /// calculates the response of the second order system
    /// at a given time
    pub fn calculate_response(&mut self, simulation_time: Time) -> f64 {

        // get the current time (t - t0)
        self.current_time = simulation_time;
        let time_elapsed = self.current_time - self.start_time;

        // first let's deal with the heaviside function

        let heaviside_on: bool = self.current_time > self.start_time;

        // if the current time is before start time, no response 
        // from this transfer function
        if !heaviside_on {
            return 0.0;
        }

        // time ratio is t/tau
        let time_ratio: Ratio = time_elapsed /  self.process_time;
        let steady_state_value: f64 = self.steady_state_value();

        // otherwise, calculate as per normal

        //// u1(t - t1) * Kp * [1-exp(- [t-t1] / tau])
        //let response: f64 = self.steady_state_value()
        //    * (1.0 - exponent_ratio.exp());
        
        // need to calculate second order response
        // which means we need the damping factor or something
        let damping_factor = self.damping_factor;

        // no unstable or undamped responses allowed
        if damping_factor <= 0.0 {
            todo!("damping factor needs to be more than 0.0, \n 
                also need to implement Result enum")
        }

        let response;

        if damping_factor < 1.0 {
            // case 1: underdamped

            let sqrt_one_minus_zeta_sq: f64 = 
                (1.0 - damping_factor.powf(2.0)).sqrt();
            // first, cos term
            // cos ( sqrt(1-zeta^2)/tau * t)

            let omega_t_term: f64 = sqrt_one_minus_zeta_sq 
                * time_ratio.get::<uom::si::ratio::ratio>();

            let cosine_term = omega_t_term.cos();
            
            // next, sine term,
            // zeta / (1 - zeta^2) * sin ( sqrt(1 - zeta^2)/ tau * t)

            let sine_term = damping_factor / sqrt_one_minus_zeta_sq 
                * omega_t_term.sin();
            
            // now we need 1 - exp(- zeta * t/tau) *
            // [ cos term + sine term ]
            
            let cosine_and_sine_term: f64 = cosine_term + sine_term;

            // exp(- zeta * t/tau) * [ cos term + sine term ]
            let exponential_term: f64 = (
                -damping_factor * time_ratio.get::<uom::si::ratio::ratio>()).exp()
                *cosine_and_sine_term;

            let scaled_response = 1.0 - exponential_term;

            // a_0 * K_p *exp(- zeta * t/tau) * [ cos term + sine term ]
            response =  steady_state_value * scaled_response;


        } else if damping_factor == 1.0 {
            // case 2: critical damping
            //
            // a_0 K_p 
            // {
            // 1 - [1 + t/tau] exp (- t/tau)
            // }

            let one_plus_t_over_tau = 
                1.0 + time_ratio.get::<uom::si::ratio::ratio>();

            let exponential_term = (
                -time_ratio.get::<uom::si::ratio::ratio>()).exp()
                * one_plus_t_over_tau;

            let scaled_response = 1.0 - exponential_term;

            response =  steady_state_value * scaled_response;

            
        } else {
            // case 3: overdamped
            
            let sqrt_zeta_sq_minus_one: f64 = 
                (damping_factor.powf(2.0) - 1.0).sqrt();

            // first, cosh term
            // cosh ( sqrt(zeta^2-1)/tau * t)

            let omega_t_term: f64 = sqrt_zeta_sq_minus_one 
                * time_ratio.get::<uom::si::ratio::ratio>();

            let cosh_term = omega_t_term.cosh();

            // next, sinh term,
            // zeta / (1 - zeta^2) * sinh ( sqrt(zeta^2 - 1)/ tau * t)

            let sinh_term = damping_factor / sqrt_zeta_sq_minus_one 
                * omega_t_term.sinh();

            // now we need 1 - exp(- zeta * t/tau) *
            // [ cosh term + sinh term ]

            let cosh_term_plus_sinh_term = cosh_term + sinh_term;

            // exp(- zeta * t/tau) * [ cos term + sine term ]
            let exponential_term: f64 = (
                -damping_factor * time_ratio.get::<uom::si::ratio::ratio>()).exp()
                *cosh_term_plus_sinh_term;

            let scaled_response = 1.0 - exponential_term;

            // a_0 * K_p *exp(- zeta * t/tau) * [ cos term + sine term ]
            response =  steady_state_value * scaled_response;
        }


        return response;
    }

    /// steady state value 
    /// u1(t - t1) * Kp 
    pub fn steady_state_value(&self) -> f64 {
        let response: f64 = self.user_input * self.process_gain;
        response
    }
}


