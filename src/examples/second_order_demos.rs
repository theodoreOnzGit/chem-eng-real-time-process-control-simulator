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


    let user_task = move ||{

        dbg!(a1);
    };

    let user_task_ptr = Arc::new(Mutex::new(user_task));
    simulation_template(user_task_ptr);
    //let tf = TransferFn::SecondOrder::new(a1, b1, c1, a2, b2, c2);
    //


}

fn simulation_template(user_task_ptr: Arc<Mutex<impl FnMut() -> ()
    + std::marker::Send + 'static>>){

    let simulation_time = Time::new::<second>(60.0);
    let mut current_time = Time::new::<second>(0.0);
    let timestep = Time::new::<millisecond>(15.0);

    let user_task_ptr_clone = user_task_ptr.clone();

    let task = move || {
        while current_time.le(&simulation_time) {

            let mut user_task_ref = user_task_ptr_clone.lock().unwrap();
            user_task_ref();

            current_time += timestep;
        }};

    let handle = thread::spawn(task);
    handle.join().unwrap();
}
