use std::sync::{Arc, Mutex};
use std::thread;

use uom::si::f64::*;
use uom::si::frequency::hertz;
use uom::si::frequency_drift::hertz_per_second;
use uom::si::ratio::ratio;
use uom::si::time::{second, millisecond};

pub(crate) fn stable_second_order(){

    use uom::si::{Quantity, ISQ, SI};
    use uom::typenum::*;
    type TimeSquaredChemEProcessControl = 
    Quantity<ISQ<Z0, Z0, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;

    let a1: TimeSquaredChemEProcessControl = 
    FrequencyDrift::new::<hertz_per_second>(1.0).recip();
    let b1: Time = Time::new::<second>(1.0);
    let c1: Ratio = Ratio::new::<ratio>(1.0);

    let a2: 
    TimeSquaredChemEProcessControl = FrequencyDrift::new::<hertz_per_second>(2.0).recip();
    let b2: Time = Time::new::<second>(2.0);
    let c2: Ratio = Ratio::new::<ratio>(2.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(60.0);
    let timestep: Time = Time::new::<millisecond>(60.0);

    //let tf = TransferFn::SecondOrder::new(a1, b1, c1, a2, b2, c2);
    //
    // if you need to set initial values
    // tf.set_initial_output(initial_value);
    //
    let stuff_to_do_in_simulation_loop = move ||{

        // let _output = tf.set_user_input_and_calc(user_input,time);
        // tf.csv_plot();
        //
        // probably want to assert something as well
        // assert approx equal 

        // should be equal within x percent of a suitable scale
        // so lets say 1e-9 times of 1,

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
