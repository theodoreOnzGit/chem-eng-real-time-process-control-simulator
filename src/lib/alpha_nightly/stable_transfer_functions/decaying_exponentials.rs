use uom::{si::{f64::*, time::second, frequency::hertz, ratio::ratio}, ConstZero};

use crate::alpha_nightly::errors::ChemEngProcessControlSimulatorError;

/// step responses for transfer function of type 
///
/// G(s) = (a1 s^2 + bs)/ (a2 s^2 + b2 s + c)
///
///
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub struct DecayingExponential {
    pub(crate) magnitude: Ratio,
    /// decay frequency or 1/decay time
    pub(crate) a: Frequency,
    pub(crate) previous_timestep_input: Ratio,
    /// oscillation frequency
    pub(crate) omega: Frequency,
    /// previous timestep output
    pub(crate) offset: Ratio,
    /// delay
    pub(crate) delay: Time,
    /// vector of first order responses 
    pub(crate) response_vec: Vec<DecayExponentialResponse>,
    /// choose whether it's a critically damped or 
    /// overdamped system
    pub(crate) sinusoid_type: DecayingExponentialType,
}

#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub enum DecayingExponentialType {
    // two distinct roots
    Overdamped, 
    // two equal real roots
    CriticallyDamped,
    // one root only
    Single,

}

/// second order response struct, 
/// will help to caluclate
/// step responses for underdamped, crtically damped and 
/// overdamped stable systems
///
///
/// for decaying exponential responses, we have two main cases 
/// the first is where there are two equal real roots. This is 
/// critical damping
///
/// The second is where we have two real unequal roots. This is 
/// overdamping. 
#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub(crate) struct DecayExponentialResponse {
    magnitude_alpha: Ratio,
    magnitude_beta: Ratio,
    alpha: Frequency,
    beta: Frequency,
    start_time: Time,
    user_input: Ratio,
    current_time: Time,
    exponential_type: DecayingExponentialType,
}

impl Default for DecayExponentialResponse {
    /// default is a critically damped system with 
    /// 1 / ( (s+1)^2 + 1)
    /// time in seconds, 
    /// frequency in hertz
    fn default() -> Self {
        DecayExponentialResponse { 
            magnitude_alpha: Ratio::new::<ratio>(1.0), 
            magnitude_beta: Ratio::new::<ratio>(1.0), 
            alpha: Frequency::new::<hertz>(1.0), 
            beta: Frequency::new::<hertz>(1.0),
            start_time: Time::new::<second>(0.0), 
            user_input: Ratio::new::<ratio>(1.0), 
            current_time: Time::new::<second>(0.0),
            exponential_type: DecayingExponentialType::CriticallyDamped,
        }
    }
}


impl DecayExponentialResponse {

    /// constructor 
    pub fn new_single_root(
        magnitude: Ratio,
        decay_constant: Frequency,
        start_time: Time,
        user_input: Ratio,
        current_time: Time) -> Result<Self,ChemEngProcessControlSimulatorError> {

        // if damping factor is less than or equal 
        // 0, should throw an error 
        // or panic (i will use errors maybe later?)

        let alpha = decay_constant;
        let beta = decay_constant;
        let exponential_type = DecayingExponentialType::Single;

        if alpha.value <= 0.0 {
            return Err(ChemEngProcessControlSimulatorError::
                UnstableDampingFactorForStableTransferFunction);
        }
        Ok(DecayExponentialResponse { 
            magnitude_alpha: magnitude, 
            magnitude_beta: magnitude, 
            alpha, 
            start_time, 
            user_input, 
            current_time,
            beta,
            exponential_type,
        })
    }

    /// checks if the transfer function has more or less reached 
    /// steady state,
    ///
    /// this is determined by exp(-at) 
    /// if at is 20 or more, then we have reached steady state
    pub fn is_steady_state(&self) -> bool {
        let time_elapsed = self.current_time - self.start_time;

        //  (at) in exp(-at)
        let at: Ratio = time_elapsed * self.alpha;
        let bt: Ratio = time_elapsed * self.beta;

        match self.exponential_type {
            DecayingExponentialType::Overdamped => {
                // need both alpha and beta to be more than 20
                if at > Ratio::new::<ratio>(20.0) && 
                bt > Ratio::new::<ratio>(20.0) {
                    return true;
                }
            },
            DecayingExponentialType::CriticallyDamped => {
                // for critically damped systems we can represent it 
                // using x exp (-x) 
                //
                // if x > 23, generally we can ignore things 
                //
                // However, this usually comes in the form: 
                //
                // t exp (-lambda t)
                //
                // so lambda t = 23 doesn't quite cut it all the time
                // we must impose an additional constraint.
                // Consider rewriting x exp (-x)
                // 1/ lambda * (lambda t) exp (-lambda t)
                //
                // my tolerance initially was for 
                // (lambda t) exp (- lambda t) \approx 1e-9
                //
                // Now I must also consider 1/lambda
                //
                // so just take the product, 

                let inverse_lambda: Time = 1.0 / self.alpha;
                let lambda_t = at;

                // i'm not going to go into specifics... but this 
                // will have to do 

                let exponent_ratio: Ratio = 
                    lambda_t * (-lambda_t.get::<ratio>()).exp() 
                    * inverse_lambda.get::<second>();
                
                if exponent_ratio < Ratio::new::<ratio>(1e-10) {
                    return true;
                }



            },
            DecayingExponentialType::Single => {
                // for single type, the condition is the same 
                // because at should equal bt if initiated properly
                if at > Ratio::new::<ratio>(20.0) && 
                bt > Ratio::new::<ratio>(20.0) {
                    return true;
                }
            },
        }


        // 

        return false;
    }


    /// calculates the response of the second order system
    /// at a given time
    pub fn calculate_response(&mut self, simulation_time: Time) -> Ratio {

        // get the current time (t - t0)
        self.current_time = simulation_time;
        let time_elapsed = self.current_time - self.start_time;

        // first let's deal with the heaviside function

        let heaviside_on: bool = self.current_time > self.start_time;

        // if the current time is before start time, no response 
        // from this transfer function
        if !heaviside_on {
            return Ratio::ZERO;
        }



        let response: Ratio;

        // for convenience, we calculate alpha t and beta t 
        let alpha_t: Ratio = time_elapsed * self.alpha;
        let beta_t: Ratio = time_elapsed * self.beta;

        response = match self.exponential_type {
            DecayingExponentialType::CriticallyDamped => {
                // for two equal roots, also quite straightforward
                // magnitude_alpha * t * exp(-alpha t)
                // magnitude_beta * exp(-beta t)
                self.magnitude_alpha * 
                    time_elapsed.get::<second>() *
                    (-alpha_t.get::<ratio>()).exp() +
                self.magnitude_beta * 
                    (-beta_t.get::<ratio>()).exp()

            },
            DecayingExponentialType::Overdamped => {
                // for two unequal roots, also quite straightforward
                // magnitude_alpha * exp(-alpha t)
                // magnitude_beta * exp(-beta t)
                self.magnitude_alpha * 
                    (-alpha_t.get::<ratio>()).exp() +
                self.magnitude_beta * 
                    (-beta_t.get::<ratio>()).exp()
            },
            DecayingExponentialType::Single => {
                // for single root, it's pretty straightforward,
                // the response is magnitude * exp(-at) 
                self.magnitude_alpha * 
                    (-alpha_t.get::<ratio>()).exp()
            },

        };

        return response;

    }

    /// steady state value 
    /// of a decaying exponential is zero
    /// eventually
    pub fn steady_state_value(&self) -> Ratio {
        let response: Ratio = Ratio::ZERO;
        response
    }
}



