use num::{Zero, One};

use crate::*;
use std::fmt;
use std::ops::{Add, Mul, Neg, Div};
pub struct Adder<T: Add + Zero + Copy + fmt::Debug, const N: usize> {
    pub inputs: [*const T;N],
    pub output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T: Add + Zero + Copy + fmt::Debug, const N: usize> Adder<T, N> {
    pub fn new(inputs:  [*const T;N]) -> Self {
        let mut sum = T::zero();
        for i in inputs {
            unsafe {
                sum = sum + *i
            }
        }
        Self {
            inputs,
            output: sum,
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, sum)]
        }
    }
    pub fn as_mut_raw(&mut self) -> *mut Self {
        self
    }
    pub fn get_output_history(&self) -> &Vec<(f64, T)> {
        &self.output_history
    }
}

impl<T: Add + Zero + Copy + fmt::Debug, const N: usize> fmt::Display for Adder<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Add + Zero + fmt::Debug + Copy, const N: usize> SimSystem for Adder<T, N> {
    fn next_step(&mut self){
        self.time += self.step_size;
        self.output = T::zero();
        
        for i in self.inputs {
            unsafe {
                self.output = self.output + *i;
            }
        }
        self.output_history.push((self.time, self.output))
    }
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}

pub struct Negator<T: Neg<Output = T> + Copy + fmt::Debug> {
    pub input: *const T,
    pub output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T: Neg<Output = T> + Copy + fmt::Debug> Negator<T> {
    pub fn new(input:  *const T) -> Self {
        unsafe {
            Self {
                input,
                output: -*input,
                time: 0.0,
                step_size: 0.1,
                output_history: vec![(0.0, -*input)]
            }
        }
    }
    pub fn as_mut_raw(&mut self) -> *mut Self {
        self
    }
    pub fn get_output_history(&self) -> &Vec<(f64, T)> {
        &self.output_history
    }
}

impl<T: Neg<Output = T> + Copy + fmt::Debug> fmt::Display for Negator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Neg<Output = T> + Copy + fmt::Debug> SimSystem for Negator<T> {
    fn next_step(&mut self){
        self.time += self.step_size;
        unsafe {
            self.output = -*self.input;
        }
        self.output_history.push((self.time, self.output))
    }
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}

pub struct MulSystem<T: Mul + One + Copy + fmt::Debug, const N: usize> {
    pub inputs: [*const T;N],
    pub output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T: Mul + One + Copy + fmt::Debug, const N: usize> MulSystem<T, N> {
    pub fn new(inputs:  [*const T;N]) -> Self {
        let mut prod = T::one();
        for i in inputs {
            unsafe {
                prod = prod * *i
            }
        }
        Self {
            inputs,
            output: prod,
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, prod)]
        }
    }
    pub fn as_mut_raw(&mut self) -> *mut Self {
        self
    }
    pub fn get_output_history(&self) -> &Vec<(f64, T)> {
        &self.output_history
    }
}

impl<T: Mul + One + Copy + fmt::Debug, const N: usize> fmt::Display for MulSystem<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Mul + One + fmt::Debug + Copy, const N: usize> SimSystem for MulSystem<T, N> {
    fn next_step(&mut self){
        self.time += self.step_size;
        self.output = T::one();
        
        for i in self.inputs {
            unsafe {
                self.output = self.output * *i;
            }
        }
        self.output_history.push((self.time, self.output))
    }
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}

pub struct Inverter<T: Div<Output = T> + One + Copy + fmt::Debug> {
    pub input: *const T,
    pub output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}


impl<T: Div<Output = T> + Copy + One + fmt::Debug> Inverter<T> {
    pub fn new(input:  *const T) -> Self {
        let mut out = T::one();
        unsafe {
            out = out / *input;
        }
        Self {
            input,
            output: out,
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, out)]
        }
    }
    pub fn as_mut_raw(&mut self) -> *mut Self {
        self
    }
    pub fn get_output_history(&self) -> &Vec<(f64, T)> {
        &self.output_history
    }
}

impl<T: Div<Output = T> + One + Copy + fmt::Debug> fmt::Display for Inverter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Div<Output = T> + One + Copy + fmt::Debug> SimSystem for Inverter<T> {
    fn next_step(&mut self){
        self.time += self.step_size;
        unsafe {
            self.output = T::one() / *self.input;
        }
        self.output_history.push((self.time, self.output))
    }
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}
