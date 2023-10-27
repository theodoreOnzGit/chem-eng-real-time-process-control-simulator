use std::sync::{Arc, Mutex};
use std::thread;

use chem_eng_real_time_process_control_simulator::beta_testing::prelude::*;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::frequency_drift::hertz_per_second;
use uom::si::ratio::ratio;
use uom::si::time::{second, millisecond};

/// 
/// This is a simulation of:
///
///         0.000119 s - 2.201 e-7
/// G(s) = ---------------------------
///         s^2 + 0.0007903 s + 6.667e-7
///
///
/// Input is in the form:
///
/// G(s) = 
///
/// a1 s^2 + b1 s + c1
/// ------------------
/// a2 s^2 + b2 s + c2
///
pub(crate) fn stable_second_order_simulation(){

    use uom::si::{Quantity, ISQ, SI};
    use uom::typenum::*;
    // type alias called TimeSquared
    type TimeSquared = 
    Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

    let one_second = Time::new::<second>(1.0);

    let a1: TimeSquared = Time::ZERO * Time::ZERO;
    let b1: Time = Time::new::<second>(0.000119);
    let c1: Ratio = -Ratio::new::<ratio>(2.201e-7);

    let a2: TimeSquared =one_second * one_second;
    let b2: Time = Time::new::<second>(0.0007903);
    let c2: Ratio = Ratio::new::<ratio>(6.667e-7);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(5.0e4 as f64);
    let timestep: Time = Time::new::<second>(200.0);

    let mut tf = TransferFnSecondOrder::new(a1, b1, c1, a2, b2, c2).unwrap();
    //
    // if you need to set initial values
    // because the transfer function only measures deviations from 
    // these inputs and outputs
    //
    // // do this before starting up
    //
    // // actually transfer functions work with deviation variables,
    // // the initial input and output is always zero
    // tf.set_dead_time(initial_value);

    let mut user_input = Ratio::ZERO;

    let stuff_to_do_in_simulation_loop = move ||{

        // let _output = tf.set_user_input_and_calc(user_input,time);
        // tf.csv_plot();
        //
        // probably want to assert something as well
        // assert approx equal 

        // should be equal within x percent of a suitable scale
        // so lets say 1e-9 times of 1,
        
        // step up to 9 if t > 0 
        if current_simulation_time > Time::ZERO {
            user_input = Ratio::new::<ratio>(9.0);
        }

        let output = tf.set_user_input_and_calc(
            user_input,current_simulation_time).unwrap();
        //dbg!(output);
        // for example
        assert_abs_diff_eq!(1.0,1.01, epsilon = 0.1);
        
        current_simulation_time += timestep;
    };

    // need to create a pointer for the stuff_to_do_in_simulation_loop
    // this is to enable parallelism
    let user_task_ptr = Arc::new(Mutex::new(stuff_to_do_in_simulation_loop));
    simulation_template(max_simulation_time, timestep, current_simulation_time,
        user_task_ptr);


}

fn simulation_template(
    max_simulation_time: Time,
    timestep: Time,
    mut current_simulation_time: Time,
    user_task_ptr: Arc<Mutex<impl FnMut() -> ()
    + std::marker::Send + 'static>>){

    let user_task_ptr_clone = user_task_ptr.clone();

    let task = move || {
        while current_simulation_time.le(&max_simulation_time) {

            let mut user_task_ref = user_task_ptr_clone.lock().unwrap();
            user_task_ref();

            current_simulation_time += timestep;
        }};

    let handle = thread::spawn(task);
    handle.join().unwrap();
}
