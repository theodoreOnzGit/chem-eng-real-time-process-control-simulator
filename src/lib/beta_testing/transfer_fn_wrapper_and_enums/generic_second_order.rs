use uom::si::ratio::ratio;
use uom::si::{f64::*, Quantity, Dimension, ISQ, SI};
use uom::{typenum::*, ConstZero};

use crate::beta_testing::TimeSquared;
use crate::beta_testing::errors::ChemEngProcessControlSimulatorError;
use crate::beta_testing::stable_transfer_functions::decaying_sinusoid::DecayingSinusoid;
use crate::beta_testing::stable_transfer_functions::second_order_transfer_fn::SecondOrderStableTransferFnNoZeroes;

/// an enum describing second order systems 
/// you are meant to put in:
///
/// G(s) = 
///
/// a1 s^2 + b1 s + c1
/// ------------------
/// a2 s^2 + b2 s + c2
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum SecondOrder {
    /// this is arranged in the order
    /// no_zero_transfer_fn,
    /// cosine_term,
    /// sine_term
    StableUnderdamped(
        SecondOrderStableTransferFnNoZeroes,
        DecayingSinusoid,DecayingSinusoid),
    StableCriticallydamped,
    StableOverdamped,
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
    c2: Ratio) -> Result<Self,ChemEngProcessControlSimulatorError> {

        // process time 
        let tau_p: Time = (a2/c2).sqrt();

        // damping factor
        let zeta: Ratio = 0.5 * b2 / (a2*c2).sqrt();

        // process_gain 
        // I assume units of c1 are dimensionless
        let k_p: Ratio = c1/c2;

        // decay constant 
        let lambda: Frequency = 0.5 *b2/a2;

        // angular frequency for decaying sinusoids
        let omega: Frequency = (c2/a2 - 0.25*b2*b2/a2/a2).sqrt();

        let zeta_value: f64 = zeta.get::<ratio>();

        if zeta_value < 0.0 {
            // unstable system
            todo!();
        } else if zeta_value < 1.0 {

        } else if zeta_value == 1.0 {
            // undamped system
            return Self::new_underdamped_stable_system(tau_p, 
                zeta, 
                k_p, lambda, omega, a1, a2, b1, b2);
        } else if zeta_value > 1.0 {
            
            // overdamped system
            todo!();
        }

        todo!()

    }

    pub fn set_dead_time(&mut self, dead_time: Time){

        match self {
            SecondOrder::StableUnderdamped(
                transfer_fn_no_zeroes, 
                cosine_term, 
                sine_term) => {
                    // have to test if this works correctly
                    transfer_fn_no_zeroes.delay = dead_time;
                    cosine_term.delay = dead_time;
                    sine_term.delay = dead_time;

            },
            SecondOrder::StableCriticallydamped => todo!(),
            SecondOrder::StableOverdamped => todo!(),
            SecondOrder::Unstable => todo!(),
            SecondOrder::Undamped => todo!(),
        }
    }

    pub fn csv_plot(&self){
        todo!()
    }

    pub fn set_user_input_and_calc(&mut self, user_input: Ratio,
        time: Time) -> Ratio {
        
        todo!()
    }

    // underdamped stable systems
    #[inline]
    fn new_underdamped_stable_system(tau_p: Time,
        zeta: Ratio,
        k_p: Ratio,
        lambda: Frequency,
        omega: Frequency,
        a1: TimeSquared,
        a2: TimeSquared,
        b1: Time,
        b2: Time) -> Result<Self, ChemEngProcessControlSimulatorError>{

        // underdamped systems will contain two decaying_sinusoid
        // types and one SecondOrderStableTransferFunction Type
        // 
        let second_order_stable_transfer_fn_no_zeroes: 
        SecondOrderStableTransferFnNoZeroes = 
        SecondOrderStableTransferFnNoZeroes::new(k_p, 
            tau_p, 
            zeta, 
            Ratio::ZERO, 
            Ratio::ZERO, 
            Time::ZERO)?;


        // let's make the decaying sinusoids first: 

        let sine_coeff: Ratio = a1/a2;
        let cosine_coeff: Ratio = -sine_coeff * (lambda - b1/a1)/omega;

        // now let's get the waveforms

        let cosine_term: DecayingSinusoid = DecayingSinusoid::new_cosine(
            cosine_coeff, 
            lambda, 
            Ratio::ZERO, 
            Ratio::ZERO, 
            Time::ZERO, 
            omega)?;

        let sine_term: DecayingSinusoid = DecayingSinusoid::new_cosine(
            sine_coeff, 
            lambda, 
            Ratio::ZERO, 
            Ratio::ZERO, 
            Time::ZERO, 
            omega)?;

        let underdamped_system = Self::StableUnderdamped(
            second_order_stable_transfer_fn_no_zeroes, cosine_term,
            sine_term);

        return Ok(underdamped_system);

    }

}


