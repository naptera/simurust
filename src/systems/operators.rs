use num::{Zero, One};

use crate::*;
use std::fmt;
use std::ops::{Add, Mul, Neg, Div};
use std::ptr::null;

#[derive(Debug, Clone)]
pub struct Adder<T: Add + Zero + Copy + fmt::Debug, const N: usize> {
    inputs: [*const T;N],
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

#[derive(Debug, Clone)]
pub struct Negator<T: Neg<Output = T> + Copy + fmt::Debug> {
    input: *const T,
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

#[derive(Debug, Clone)]
pub struct Multiplier<T: Mul + One + Zero + Copy + fmt::Debug, const N: usize> {
    inputs: [*const T;N],
    output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T: Mul + One + Zero + Copy + fmt::Debug, const N: usize> Multiplier<T, N> {
    pub fn new() -> Self {
        Self {
            inputs: [null();N],
            output: T::zero(),
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, T::zero())]
        }
    }

    pub fn get_output_history(&self) -> &Vec<(f64, T)> {
        &self.output_history
    }
}

impl<T: Mul + One + Zero + Copy + fmt::Debug, const N: usize> fmt::Display for Multiplier<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Mul + One + Zero + fmt::Debug + Copy, const N: usize> SimSystem for Multiplier<T, N> {
    fn next_step(&mut self){
        self.time += self.step_size;
        self.output = T::one();
        
        for i in self.inputs {
            if i.is_null() {
                self.output = T::zero();
                break;
            } else { 
                unsafe {
                    self.output = self.output * *i;
                }
            }
        }
        self.output_history.push((self.time, self.output))
    }
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}

impl<T, const N: usize> Source<T> for Multiplier<T, N> 
where T: Mul + One + Zero + fmt::Debug + Copy {
    fn get_output_ref(self: &Self, _port: usize) -> &T {
        &self.output
    }
}

impl<T, const N: usize> Sink<T> for Multiplier<T, N>
where T: Mul + One + Zero + fmt::Debug + Copy {
    fn set_input(self: &mut Self, input: *const T, port: usize) {
        self.inputs[port] = input;
    }
}

#[derive(Debug, Clone)]
pub struct Inverter<T: Div<Output = T> + One + Copy + fmt::Debug> {
    input: *const T,
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

impl<T> Source<T> for Inverter<T>
where T: Div<Output = T> + One + Copy + fmt::Debug {
    fn get_output_ref(self: &Self, _port: usize) -> &T {
        &self.output
    }
}

impl<T> Sink<T> for Inverter<T>
where T: Div<Output = T> + One + Copy + fmt::Debug {
    fn set_input(self: &mut Self, input: *const T, _port: usize) {
        self.input = input;
    }
}

#[derive(Debug, Clone)]
pub struct Gain<T: Mul<Output = T> + Zero + Copy + fmt::Debug> {
    gain: T,
    input: *const T,
    output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T> Gain<T>
    where T: Mul<Output = T> + Zero + Copy + fmt::Debug 
{
    pub fn new() -> Self where T: One {
        Self {
            gain: T::one(),
            input: null(),
            output: T::zero(),
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, T::zero())]
        }
    }
    
    pub fn from(input: *const T, gain: T, start_time: f64, step_size: f64) -> Self {
        unsafe {
            let output = *input * gain;
            Self {
                gain,
                input,
                output,
                time: start_time,
                step_size,
                output_history: vec![(0.0, output)]
            }
        }
    }

    pub fn set_input(self: &mut Self, input: *const T) {
        self.input = input;
    }

    pub fn get_output_ref(self: &Self) -> &T {
        &self.output
    }

    pub fn get_output(self: &Self) -> T {
        self.output
    }
}

impl<T> fmt::Display for Gain<T>
    where T: Mul<Output = T> + Zero + Copy + fmt::Debug 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T> SimSystem for Gain<T>
    where T: Mul<Output = T> + Zero + Copy + fmt::Debug 
{
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }

    fn next_step(&mut self) {
        self.time += self.step_size;
        if self.input.is_null() {
                self.output = T::zero();
        } else {
            unsafe {
                self.output = *self.input * self.gain;
            }
        }
        self.output_history.push((self.time, self.output));
    }
}

impl<T> Sink<T> for Gain<T>
where T: Mul<Output = T> + Zero + Copy + fmt::Debug  {
    fn set_input(self: &mut Self, input: *const T, _port: usize) {
        self.input = input;
    }
}

impl<T> Source<T> for Gain<T>
where T: Mul<Output = T> + Zero + Copy + fmt::Debug  {
    fn get_output_ref(self: &Self, _port: usize) -> &T {
        &self.output
    }
}

#[derive(Debug, Clone)]
pub struct Differentiator {
    prev_input: f64,
    input: *const f64,
    pub output: f64,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, f64)>,
    sqrteps: f64,
}

impl Differentiator {
    pub fn new(input: *const f64) -> Self {
        let eps = f64::EPSILON;
        unsafe {
            Self {
                prev_input: 0.0,
                input,
                output: *input / eps,
                time: 0.0,
                step_size: 0.1,
                output_history: vec![(0.0, 0.0)],
                sqrteps: eps.sqrt()
            }
        }
    }
}

impl fmt::Display for Differentiator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl SimSystem for Differentiator {
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }

    fn next_step(&mut self) {
        self.time += self.step_size;
        unsafe {
            self.output = (*self.input - self.prev_input) / self.step_size;
            self.prev_input = *self.input;
        }
        self.step_size = self.sqrteps * self.time;
        self.output_history.push((self.time, self.output));
    }
}
