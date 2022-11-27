use crate::*;
use num::{pow::pow, Zero, One, NumCast, traits::Pow};
use std::{fmt, ops::{Mul, Add}};

#[derive(Debug, Clone)]
pub struct Polynomial<T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> {
    coefficients: [T;N],
    pub output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> Polynomial<T, N> {
    pub fn new() -> Self {
        let mut c = [T::zero();N];
        c[0] = T::zero();
        c[1] = T::one();
        Self {
            coefficients: c,
            output: T::zero(),
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, T::zero())]
        }
    }
    pub fn from(coefficients: [T;N], start_time: f64, step_size: f64) -> Self {
        Self {
            coefficients,
            output: coefficients[0],
            time: start_time,
            step_size,
            output_history: vec![(start_time, coefficients[0])]
        }
    }
    pub fn get_output_history(&self) -> &Vec<(f64, T)> {
        &self.output_history
    }
}

impl<T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> fmt::Display for Polynomial<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> SimSystem for Polynomial<T, N> {
    fn next_step(&mut self) {
        self.time += self.step_size;
        self.output = T::zero();
        for i in 0..self.coefficients.len() {
            self.output = self.output + self.coefficients[i]*pow(T::from(self.time).unwrap(),i);
        }
        self.output_history.push((self.time, self.output))
    }

    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}


#[derive(Debug, Clone)]
pub struct Exponential<T: Pow<f64, Output = T> + One + Copy + fmt::Debug> {
    base: T,
    pub output: T,
    time: f64,
    step_size: f64,
    output_history: Vec<(f64, T)>
}

impl<T: Pow<f64, Output = T> + One + Copy + fmt::Debug> Exponential<T> {
    pub fn new() -> Self where T: NumCast{
        Self {
            base: T::from(1.0_f64.exp()).unwrap(),
            output: T::one(),
            time: 0.0,
            step_size: 0.1,
            output_history: vec![(0.0, T::one())]
        }
    }
    pub fn from(base: T, start_time: f64, step_size: f64) -> Self {
        Self {
            base,
            output: T::one(),
            time: start_time,
            step_size,
            output_history: vec![(start_time, T::one())]
        }
    }
}

impl<T: Pow<f64, Output = T> + One + Copy + fmt::Debug> fmt::Display for Exponential<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.output_history)
    }
}

impl<T: Pow<f64, Output = T> + One + Copy + fmt::Debug> SimSystem for Exponential<T> {
    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }

    fn next_step(&mut self) {
        self.time += self.step_size;
        self.output = self.base.pow(self.time);
        self.output_history.push((self.time, self.output));
    }
}
