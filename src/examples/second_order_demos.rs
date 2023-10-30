use std::sync::{Arc, Mutex};
use std::thread;

use chem_eng_real_time_process_control_simulator::alpha_nightly::prelude::*;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::si::time::second;

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
    let max_simulation_time: Time = Time::new::<second>(4.0e4 as f64);
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

    // writer creation

    let mut wtr = tf.spawn_writer("one_zero_two_complex_poles_demo_".to_string()).unwrap();

    let stuff_to_do_in_simulation_loop = move ||{

        // let _output = tf.set_user_input_and_calc(user_input,time);
        // tf.csv_write_values();
        //
        // probably want to assert something as well
        // assert approx equal 

        // should be equal within x percent of a suitable scale
        // so lets say 1e-9 times of 1,
        
        // step up to 9 if t >= 0 
        if current_simulation_time >= Time::ZERO {
            user_input = Ratio::new::<ratio>(9.0);
        }

        let output = tf.set_user_input_and_calc(
            user_input,current_simulation_time).unwrap();
        //dbg!(output);
        // assert example
        assert_abs_diff_eq!(1.0,1.01, epsilon = 0.1);
        let writer_borrow = &mut wtr;
        tf.csv_write_values(writer_borrow, current_simulation_time, 
            user_input, output).unwrap();
        //let current_time_string = current_simulation_time.get::<second>().to_string();
        //let input_string = user_input.get::<ratio>().to_string();
        //let output_string = output.get::<ratio>().to_string();

        
        //wtr.write_record(&[current_time_string,
        //    input_string,
        //    output_string]).unwrap();
        //wtr.flush().unwrap();
        
        current_simulation_time += timestep;
    };

    // need to create a pointer for the stuff_to_do_in_simulation_loop
    // this is to enable parallelism
    let user_task_ptr = Arc::new(Mutex::new(stuff_to_do_in_simulation_loop));
    simulation_template(max_simulation_time, timestep, current_simulation_time,
        user_task_ptr);


}


/// 
/// This is a simulation of:
///
///         2.5
/// G(s) = ---------------------------
///         3 s^2 + 4 s + 4
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
pub(crate) fn no_zeroes_stable_underdamped_second_order_simulation(){

    use uom::si::{Quantity, ISQ, SI};
    use uom::typenum::*;
    // type alias called TimeSquared
    type TimeSquared = 
    Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

    let one_second = Time::new::<second>(1.0);

    let a1: TimeSquared = Time::ZERO * Time::ZERO;
    let b1: Time = Time::ZERO;
    let c1: Ratio = Ratio::new::<ratio>(2.5);

    let a2: TimeSquared =one_second * one_second* 3.0;
    let b2: Time = Time::new::<second>(4.0);
    let c2: Ratio = Ratio::new::<ratio>(4.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(30 as f64);
    let timestep: Time = Time::new::<second>(0.1);

    let mut tf = TransferFnSecondOrder::new(a1, b1, c1, a2, b2, c2).unwrap();
    //
    let mut user_input = Ratio::ZERO;

    // writer creation

    let mut wtr = tf.spawn_writer("demo_no_zeroes_".to_string()).unwrap();

    let stuff_to_do_in_simulation_loop = move ||{

        
        // step up to 5 if t > 0 
        if current_simulation_time >= Time::ZERO {
            user_input = Ratio::new::<ratio>(5.0);
        }

        let output = tf.set_user_input_and_calc(
            user_input,current_simulation_time).unwrap();

        let writer_borrow = &mut wtr;
        tf.csv_write_values(writer_borrow, current_simulation_time, 
            user_input, output).unwrap();
        
        current_simulation_time += timestep;
    };

    // need to create a pointer for the stuff_to_do_in_simulation_loop
    // this is to enable parallelism
    let user_task_ptr = Arc::new(Mutex::new(stuff_to_do_in_simulation_loop));
    simulation_template(max_simulation_time, timestep, current_simulation_time,
        user_task_ptr);


}

/// 
/// This is a simulation of:
///
///         2.5s
/// G(s) = ---------------------------
///         3 s^2 + 4 s + 4
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
pub(crate) fn decaying_sine_stable_underdamped_second_order_simulation(){

    use uom::si::{Quantity, ISQ, SI};
    use uom::typenum::*;
    // type alias called TimeSquared
    type TimeSquared = 
    Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

    let one_second = Time::new::<second>(1.0);

    let a1: TimeSquared = Time::ZERO * Time::ZERO;
    let b1: Time = Time::new::<second>(2.5);
    let c1: Ratio = Ratio::new::<ratio>(0.0);

    let a2: TimeSquared =one_second * one_second* 3.0;
    let b2: Time = Time::new::<second>(4.0);
    let c2: Ratio = Ratio::new::<ratio>(4.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(30 as f64);
    let timestep: Time = Time::new::<second>(0.1);

    let mut tf = TransferFnSecondOrder::new(a1, b1, c1, a2, b2, c2).unwrap();
    //
    let mut user_input = Ratio::ZERO;

    // writer creation

    let mut wtr = tf.spawn_writer("demo_decay_sine_".to_string()).unwrap();

    let stuff_to_do_in_simulation_loop = move ||{

        
        // step up to 5 if t > 0 
        if current_simulation_time >= Time::ZERO {
            user_input = Ratio::new::<ratio>(5.0);
        }

        let output = tf.set_user_input_and_calc(
            user_input,current_simulation_time).unwrap();

        let writer_borrow = &mut wtr;
        tf.csv_write_values(writer_borrow, current_simulation_time, 
            user_input, output).unwrap();
        
        current_simulation_time += timestep;
    };

    // need to create a pointer for the stuff_to_do_in_simulation_loop
    // this is to enable parallelism
    let user_task_ptr = Arc::new(Mutex::new(stuff_to_do_in_simulation_loop));
    simulation_template(max_simulation_time, timestep, current_simulation_time,
        user_task_ptr);


}
/// This is a simulation of:
///
///         2.5s^2 - 0.5 s + 1
/// G(s) = ---------------------------
///         3 s^2 + 4 s + 4
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
pub(crate) fn demo_complex_stable_underdamped_second_order_simulation(){

    use uom::si::{Quantity, ISQ, SI};
    use uom::typenum::*;
    // type alias called TimeSquared
    type TimeSquared = 
    Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

    let one_second = Time::new::<second>(1.0);

    let a1: TimeSquared = one_second * Time::new::<second>(2.5);
    let b1: Time = -Time::new::<second>(0.5);
    let c1: Ratio = Ratio::new::<ratio>(1.0);

    let a2: TimeSquared =one_second * one_second* 3.0;
    let b2: Time = Time::new::<second>(4.0);
    let c2: Ratio = Ratio::new::<ratio>(4.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(30 as f64);
    let timestep: Time = Time::new::<second>(0.1);

    let mut tf = TransferFnSecondOrder::new(a1, b1, c1, a2, b2, c2).unwrap();
    //
    let mut user_input = Ratio::ZERO;

    // writer creation

    let mut wtr = tf.spawn_writer("demo_complex_second_order_".to_string()).unwrap();

    let stuff_to_do_in_simulation_loop = move ||{

        
        // step up to 5 if t > 0 
        if current_simulation_time >= Time::ZERO {
            user_input = Ratio::new::<ratio>(5.0);
        }

        let output = tf.set_user_input_and_calc(
            user_input,current_simulation_time).unwrap();

        let writer_borrow = &mut wtr;
        tf.csv_write_values(writer_borrow, current_simulation_time, 
            user_input, output).unwrap();
        
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
