use num::Zero;

use crate::*;
use std::fmt;
use std::ops::Add;
pub struct AddSystem<'a, T: Add + Zero + Copy + fmt::Debug, const N: usize> {
    name: &'a str,
    pub inputs: [*const T;N],
    pub output: *mut T,
    time: f64,
    step_size: f64,
    output_history: Vec<Vec<(f64, T)>>
}

impl<'a, T: Add + Zero + Copy + fmt::Debug, const N: usize> AddSystem<'a, T, N> {
    pub fn new(name: &'a str, inputs:  [*const T;N], output: *mut T) -> Self {
        Self {
            name,
            inputs,
            output,
            time: 0.0,
            step_size: 0.1,
            output_history: vec![Vec::new()]
        }
    }
    pub fn as_mut_raw(&mut self) -> *mut Self {
        self
    }
    pub fn get_output_history(&self) -> &Vec<Vec<(f64, T)>> {
        &self.output_history
    }
}

impl<'a, T: Add + Zero + Copy + fmt::Debug, const N: usize> fmt::Display for AddSystem<'a, T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:#?}", self.name, self.output_history)
    }
}

impl<'a, T: Add + Zero + fmt::Debug + Copy, const N: usize> SimSystem for AddSystem<'a, T, N> {
    fn next_step(&mut self){
        self.time += self.step_size;
        unsafe {
            *self.output = T::zero();
            for i in &self.inputs {
                *self.output = *self.output + **i;
            }
            self.output_history[0].push((self.time, *self.output))
        }
    }
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}

