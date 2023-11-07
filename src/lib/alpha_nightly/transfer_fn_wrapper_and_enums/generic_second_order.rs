use csv::Writer;
use uom::si::ratio::ratio;
use uom::si::f64::*;
use uom::si::time::second;
use uom::ConstZero;

use crate::alpha_nightly::{TimeSquared, stable_transfer_functions::decaying_exponentials::DecayingExponential};
use crate::alpha_nightly::errors::ChemEngProcessControlSimulatorError;
use crate::alpha_nightly::stable_transfer_functions::decaying_sinusoid::DecayingSinusoid;
use crate::alpha_nightly::stable_transfer_functions::second_order_transfer_fn::SecondOrderStableTransferFnNoZeroes;

use super::{TransferFn, TransferFnTraits};

/// an enum describing generic second order systems,
/// only stable systems are implemented so far
///
/// you are meant to put in:
///
/// G(s) = 
///
/// a1 s^2 + b1 s + c1
/// ------------------
/// a2 s^2 + b2 s + c2
#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum TransferFnSecondOrder {
    /// this is arranged in the order
    /// no_zero_transfer_fn,
    /// cosine_term,
    /// sine_term
    StableUnderdamped(
        SecondOrderStableTransferFnNoZeroes,
        DecayingSinusoid,DecayingSinusoid),
    StableCriticallydamped(SecondOrderStableTransferFnNoZeroes,
        DecayingExponential),
    StableOverdamped(SecondOrderStableTransferFnNoZeroes,
        DecayingExponential),
    Unstable,
    Undamped,
}

impl Default for TransferFnSecondOrder {
    fn default() -> Self {
        todo!()
    }
}

impl TransferFnTraits for TransferFnSecondOrder {
    fn set_dead_time(&mut self, dead_time: Time){

        match self {
            TransferFnSecondOrder::StableUnderdamped(
                transfer_fn_no_zeroes, 
                cosine_term, 
                sine_term) => {
                    // have to test if this works correctly
                    transfer_fn_no_zeroes.delay = dead_time;
                    cosine_term.delay = dead_time;
                    sine_term.delay = dead_time;

            },
            TransferFnSecondOrder::StableCriticallydamped(
                non_zero_steady_state_mode,decaying_mode) => {
                non_zero_steady_state_mode.delay = dead_time;
                decaying_mode.delay = dead_time;
            },
            TransferFnSecondOrder::StableOverdamped(
                non_zero_steady_state_mode,decaying_mode) => {
                non_zero_steady_state_mode.delay = dead_time;
                decaying_mode.delay = dead_time;
            },
            TransferFnSecondOrder::Unstable => todo!(),
            TransferFnSecondOrder::Undamped => todo!(),
        }
    }


    fn set_user_input_and_calc(&mut self, user_input: Ratio,
        time: Time) -> 
        Result<Ratio, ChemEngProcessControlSimulatorError> {

            match self {
                TransferFnSecondOrder::StableUnderdamped(
                    tf_no_zeroes, 
                    cosine_decaying_sinusoid, 
                    sine_decaying_sinusoid_) => {
                    let mut response: Ratio = Ratio::ZERO;

                    let tf_no_zeroes_output = 
                        tf_no_zeroes.set_user_input_and_calc_output(
                            time, user_input)?;
                    let cosine_decaying_output = 
                        cosine_decaying_sinusoid.set_user_input_and_calc_output(
                            time, user_input)?;
                    let sine_decaying_output = 
                        sine_decaying_sinusoid_.set_user_input_and_calc_output(
                            time, user_input)?;

                    //dbg!(sine_decaying_output);
                    //dbg!(cosine_decaying_output);
                    //dbg!(tf_no_zeroes_output);

                    response += tf_no_zeroes_output;
                    response += cosine_decaying_output;
                    response += sine_decaying_output;
                    return Ok(response);

                },
                TransferFnSecondOrder::StableCriticallydamped(
                    non_zero_steady_state_mode,decaying_mode)=> {
                    let mut response: Ratio = Ratio::ZERO;

                    let non_zero_steady_state_output = 
                        non_zero_steady_state_mode.set_user_input_and_calc_output(
                            time, user_input)?;
                    let decaying_output = 
                        decaying_mode.set_user_input_and_calc_output(
                            time, user_input)?;
                    response += non_zero_steady_state_output;
                    response += decaying_output;

                    return Ok(response);


                },
                TransferFnSecondOrder::StableOverdamped( 
                    non_zero_steady_state_mode,decaying_mode)=> {
                    let mut response: Ratio = Ratio::ZERO;

                    let non_zero_steady_state_output = 
                        non_zero_steady_state_mode.set_user_input_and_calc_output(
                            time, user_input)?;
                    let decaying_output = 
                        decaying_mode.set_user_input_and_calc_output(
                            time, user_input)?;
                    response += non_zero_steady_state_output;
                    response += decaying_output;

                    return Ok(response);
                },
                TransferFnSecondOrder::Unstable => todo!(),
                TransferFnSecondOrder::Undamped => todo!(),
            }

    }

    fn spawn_writer(&mut self, name: String) -> Result<Writer<std::fs::File>,
        ChemEngProcessControlSimulatorError>{
            let mut title_string: String = name;
            match self {
                TransferFnSecondOrder::StableUnderdamped(_, _, _) => {
                    title_string += "2nd_ord_transfer_fn_stable_underdamped.csv";
                },
                TransferFnSecondOrder::StableCriticallydamped(_,_) => {
                    title_string += "2nd_ord_transfer_fn_stable_critdamped.csv";
                },
                TransferFnSecondOrder::StableOverdamped(_,_) => {
                    title_string += "2nd_ord_transfer_fn_stable_overdamped.csv";
                },
                TransferFnSecondOrder::Unstable => todo!(),
                TransferFnSecondOrder::Undamped => todo!(),
            }
            let wtr = Writer::from_path(title_string)?;
            Ok(wtr)
    }

    fn csv_write_values(&mut self, 
        wtr: &mut Writer<std::fs::File>,
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

impl TransferFnSecondOrder {


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

        // damping factor
        let zeta_value: f64 = zeta.get::<ratio>();

        if zeta_value < 0.0 {
            // unstable system
            todo!("unstable system, not implemented");
        } else if zeta_value < 1.0 {
            // angular frequency for decaying sinusoids IF we have 
            // an underdamped system
            let omega: Frequency = (c2/a2 - 0.25*b2*b2/a2/a2).sqrt();
        // decay constant 
        // note that this only applies for critical and underdamped cases
            let lambda: Frequency = 0.5 *b2/a2;
            // underdamped system
            return Self::new_underdamped_stable_system(tau_p, 
                zeta, 
                k_p, lambda, omega, a1, a2, b1);

        } else if zeta_value == 1.0 {
            // decay constant 
            // note that this only applies for critical and underdamped cases
            let lambda: Frequency = 0.5 *b2/a2;
            // critically damped system, not implemented yet
            return Self::new_critdamped_stable_system(
                tau_p, zeta, k_p, lambda, a1, a2, b1);
        } else {

            // overdamped system
            todo!("overdamped system, not implemented yet");
        }


    }

    // critically damped stable systems
    #[inline]
    fn new_critdamped_stable_system(tau_p: Time,
        zeta: Ratio,
        k_p: Ratio,
        lambda: Frequency,
        a1: TimeSquared,
        a2: TimeSquared,
        b1: Time) -> Result<Self, ChemEngProcessControlSimulatorError>{

        // supposing has zeta, k_p, lambda
        // where lambda is 0.5 a2/b2
        //
        // we have three terms to deal with. 
        // Firstly the standard second order stable transfer fn
        // which deals with the c1 term
        //
        // remember the form 
        // (a1 s^2 + b1 s + c1)/(a2 s^2 + b2 s + c2)
        //
        // The c1 term represents the part with no zeroes,
        // which has a steady state value
        //
        // the other term consists of decaying exponentials

        let second_order_stable_transfer_fn_no_zeroes: 
            SecondOrderStableTransferFnNoZeroes = 
            SecondOrderStableTransferFnNoZeroes::new(k_p, 
                tau_p, 
                zeta, 
                Ratio::ZERO, 
                Ratio::ZERO, 
                Time::ZERO)?;

        // next is to deal with the decaying exponential terms 
        // which are in a1s + b
        // the overall coefficient is a1/a2 
        //
        // Laplace of { (a1s + b)/(a2 s^2 + bs + c) } 
        // = a1/a2 { exp (-lambda t) + (b1/a1 - lambda) 
        // * t exp(- lambda t)} 
        //
        // the overall coefficient is a1/a2 
        // it is also the exponential coefficient


        let exponential_coefficient: Ratio = a1/a2;
        let t_exponential_coefficient: Frequency 
            = b1/a2 - lambda * a1/a2;

        // now I need to create a new decaying exponential 
        // no delays are given
        let crit_decaying_exponential = DecayingExponential::new_critical(
            t_exponential_coefficient, 
            exponential_coefficient, 
            lambda, 
            Ratio::ZERO, 
            Ratio::ZERO, 
            Time::ZERO)?;

        // now combine them in the enum 

        return Ok(
            TransferFnSecondOrder::StableCriticallydamped(
                second_order_stable_transfer_fn_no_zeroes, 
                crit_decaying_exponential)
            );


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
        b1: Time) -> Result<Self, ChemEngProcessControlSimulatorError>{

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

        let cosine_coeff: Ratio = a1/a2;
        let sine_coeff: Ratio = -cosine_coeff * lambda/omega + 
            b1/a2/omega;

        // now let's get the waveforms

        let cosine_term: DecayingSinusoid = DecayingSinusoid::new_cosine(
            cosine_coeff, 
            lambda, 
            Ratio::ZERO, 
            Ratio::ZERO, 
            Time::ZERO, 
            omega)?;

        let sine_term: DecayingSinusoid = DecayingSinusoid::new_sine(
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


#[test]
pub fn test_dead_time(){
    use uom::si::time::second;
    use uom::si::frequency_drift::hertz_per_second;
    use uom::si::{Quantity, ISQ, SI};
    use uom::typenum::*;


    /// G(s) = exp(-5s)
    ///
    /// 4 s^2 + 5 s + 6
    /// ------------------
    /// a2 s^2 + b2 s + c2
    type TimeSquared = 
        Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

    let a1: TimeSquared = 
        FrequencyDrift::new::<hertz_per_second>(1.0).recip();
    let b1: Time = Time::new::<second>(1.0);
    let c1: Ratio = Ratio::new::<ratio>(1.0);

    let a2: 
        TimeSquared = FrequencyDrift::new::<hertz_per_second>(1.0).recip();
    let b2: Time = Time::new::<second>(1.0);
    let c2: Ratio = Ratio::new::<ratio>(2.0);

    let mut tf: TransferFn = 
        TransferFnSecondOrder::new(a1, b1, c1, a2, b2, c2).unwrap().into();

    let dead_time = Time::new::<second>(5.0);
    tf.set_dead_time(dead_time);

    // i need to match two enums, but I'm only going to use if let
    if let TransferFn::SecondOrder(second_order) = tf {
        if let TransferFnSecondOrder::StableUnderdamped(
            tf_no_zeroes, cosine_sinusoidal_decay, sine_sinusoidal_decay) = second_order {
            assert_eq!(tf_no_zeroes.delay, dead_time);
            assert_eq!(cosine_sinusoidal_decay.delay, dead_time);
            assert_eq!(sine_sinusoidal_decay.delay, dead_time);
        }
    }


}

impl Into<TransferFn> for TransferFnSecondOrder {
    fn into(self) -> TransferFn {
        TransferFn::SecondOrder(self)
    }
}

impl TryFrom<TransferFn> for TransferFnSecondOrder {
    type Error = ChemEngProcessControlSimulatorError;
    fn try_from(generic_transfer_function: TransferFn) 
        -> Result<Self, Self::Error> {

            if let TransferFn::SecondOrder(
                second_order) = generic_transfer_function {
                return Ok(second_order);
            } else {
                return Err(ChemEngProcessControlSimulatorError::WrongTransferFnType);
            };


        }
}

