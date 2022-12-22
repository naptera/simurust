pub mod systems;
use num::{NumCast};
use std::{fmt::Display};

pub fn cast<U: NumCast, V: NumCast>(value: U) -> V {
    V::from(value).expect("Some types are not castable between each other")
}

pub trait SimSystem: Display {
    fn next_step(&mut self);
    fn get_next_time(&self) -> f64;
}

pub trait Source<T>: SimSystem {
    fn get_output_ref(self: &Self, port: usize) -> &T;
}

pub trait Sink<T>: SimSystem {
    fn set_input(self: &mut Self, input: *const T, port: usize);
}

#[macro_export]
macro_rules! connect {
    ($a: expr, $i: expr, $b: expr, $j: expr) => {
        $b.set_input($a.get_output_ref($i), $j)
    };
}

pub fn run_simulation<const N: usize>(systems: [*mut dyn SimSystem;N], start_time: f64, stop_time: f64) -> f64 {
    let mut time = start_time;
    if stop_time <= 0.0 {
        return time;
    }
    unsafe {
        loop {
        
            let mut min_time = systems[0].as_ref().unwrap().get_next_time();
            let mut index: usize = 0;
            for i in 1..N {
                if systems[i].as_ref().unwrap().get_next_time() < min_time {
                    index = i;
                    min_time = systems[i].as_ref().unwrap().get_next_time();
                }
            }
            if min_time > stop_time {
                break;
            }
            systems[index].as_mut().unwrap().next_step();
            time = min_time;
        }
    }
    time
}


