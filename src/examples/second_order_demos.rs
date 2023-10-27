use std::sync::{Arc, Mutex};
use std::thread;

use uom::si::f64::*;
use uom::si::frequency::hertz;
use uom::si::frequency_drift::hertz_per_second;
use uom::si::ratio::ratio;
use uom::si::time::{second, millisecond};


pub(crate) fn stable_second_order(){


    let a1: FrequencyDrift = FrequencyDrift::new::<hertz_per_second>(1.0);
    let b1: Frequency = Frequency::new::<hertz>(1.0);
    let c1: Ratio = Ratio::new::<ratio>(1.0);

    let a2: FrequencyDrift = FrequencyDrift::new::<hertz_per_second>(2.0);
    let b2: Frequency = Frequency::new::<hertz>(2.0);
    let c2: Ratio = Ratio::new::<ratio>(2.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(60.0);
    let timestep: Time = Time::new::<millisecond>(60.0);


    //let tf = TransferFn::SecondOrder::new(a1, b1, c1, a2, b2, c2);
    //
    let stuff_to_do_in_simulation_loop = move ||{

        // transferfn.set_user_input_and_calc(user_input,time);
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
