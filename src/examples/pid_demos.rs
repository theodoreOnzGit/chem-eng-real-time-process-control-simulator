/// Copyright [2023] [Theodore Kay Chen Ong, Professor Per F. Peterson,
/// University of California, Berkeley
/// Thermal Hydraulics Lab, Repository Contributors and 
/// Singapore Nuclear Research and Safety Initiative (SNRSI)]
/// 
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
use std::sync::{Arc, Mutex};
use std::thread;

use chem_eng_real_time_process_control_simulator::alpha_nightly::prelude::*;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::si::time::second;

/// 
/// This is a simulation of three step changes for an integral 
/// controller 1/s
pub(crate) fn proportional_integral_test(){

    let one_second = Time::new::<second>(1.0);

    let integral_time: Time = one_second;
    let controller_gain = Ratio::new::<ratio>(1.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(60 as f64);
    let timestep: Time = Time::new::<second>(0.2);

    let mut integral_controller: Controller = 
        IntegralController::new(controller_gain,
            integral_time).unwrap().into();

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

    let mut wtr = integral_controller.spawn_writer("demo_ramp_fn".to_string()).unwrap();

    let stuff_to_do_in_simulation_loop = move ||{
        // for this case, I have three step functions 
        // the step functions are:
        //
        // at 5s to 10s, input is 1 
        // at at 10s to 15s, input is 2.5
        // at 15s onwards, input is -1

        if current_simulation_time <= Time::ZERO {
            // do nothing, leave it at zero
        } else if current_simulation_time < Time::new::<second>(5.0) {
            user_input = Ratio::ZERO;
        } else if current_simulation_time < Time::new::<second>(10.0) {
            user_input = Ratio::new::<ratio>(1.0);
        } else if current_simulation_time < Time::new::<second>(15.0) {
            user_input = Ratio::new::<ratio>(2.5);
        } else {
            user_input = Ratio::new::<ratio>(-1.0);
        }

        let output = integral_controller.set_user_input_and_calc(
            user_input, current_simulation_time).unwrap();

        // write 
        let writer_borrow = &mut wtr;
        integral_controller.csv_write_values(
            writer_borrow, current_simulation_time, 
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
/// This is a simulation of three step changes for an integral 
/// controller 1/s
pub(crate) fn integral_controller_ramp_test(){

    let one_second = Time::new::<second>(1.0);

    let integral_time: Time = one_second;
    let controller_gain = Ratio::new::<ratio>(1.0);
    let mut current_simulation_time: Time = Time::new::<second>(0.0);
    let max_simulation_time: Time = Time::new::<second>(60 as f64);
    let timestep: Time = Time::new::<second>(0.2);

    let mut integral_controller: Controller = 
        IntegralController::new(controller_gain,
            integral_time).unwrap().into();

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

    let mut wtr = integral_controller.spawn_writer("demo_ramp_fn".to_string()).unwrap();

    let stuff_to_do_in_simulation_loop = move ||{
        // for this case, I have three step functions 
        // the step functions are:
        //
        // at 5s to 10s, input is 1 
        // at at 10s to 15s, input is 2.5
        // at 15s onwards, input is -1

        if current_simulation_time <= Time::ZERO {
            // do nothing, leave it at zero
        } else if current_simulation_time < Time::new::<second>(5.0) {
            user_input = Ratio::ZERO;
        } else if current_simulation_time < Time::new::<second>(10.0) {
            user_input = Ratio::new::<ratio>(1.0);
        } else if current_simulation_time < Time::new::<second>(15.0) {
            user_input = Ratio::new::<ratio>(2.5);
        } else {
            user_input = Ratio::new::<ratio>(-1.0);
        }

        let output = integral_controller.set_user_input_and_calc(
            user_input, current_simulation_time).unwrap();

        // write 
        let writer_borrow = &mut wtr;
        integral_controller.csv_write_values(
            writer_borrow, current_simulation_time, 
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

