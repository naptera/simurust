use crate::*;
use num::{pow::pow, Zero, One, NumCast};
use std::{fmt, ops::{Mul, Add}};
#[derive(Debug)]
pub struct PolynomialSource<'a, T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> {
    name: &'a str,
    coefficients: [T;N],
    pub output: *mut T,
    time: f64,
    step_size: f64,
    output_history: Vec<Vec<(f64, T)>>
}

impl<'a, T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> PolynomialSource<'a, T, N> {
    pub fn new(name: &'a str, output: *mut T) -> Self {
        let mut c = [T::zero();N];
        c[0] = T::zero();
        c[1] = T::one();
        Self {
            name,
            coefficients: c,
            output,
            time: 0.0,
            step_size: 0.1,
            output_history: vec![vec![(0.0, T::zero())]]
        }
    }
    pub fn from(name: &'a str, output: *mut T, coefficients: [T;N], start_time: f64, step_size: f64) -> Self {
        Self {
            name,
            coefficients,
            output,
            time: start_time,
            step_size,
            output_history: vec![vec![(start_time, coefficients[0])]]
        }
    }
    pub fn get_output_history(&self) -> &Vec<Vec<(f64, T)>> {
        &self.output_history
    }
}

impl<'a, T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> fmt::Display for PolynomialSource<'a, T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:#?}", self.name, self.output_history)
    }
}

impl<'a, T: Mul<T, Output = T> + Add + Zero + Copy + fmt::Debug + NumCast + One, const N: usize> SimSystem for PolynomialSource<'a, T, N> {
    fn next_step(&mut self) {
        self.time += self.step_size;
        unsafe {
            *self.output = T::zero();
            for i in 0..self.coefficients.len() {
                *self.output = *self.output + self.coefficients[i]*pow(T::from(self.time).unwrap(),i);
            }
        }
    }

    fn get_next_time(&self) -> f64 {
        self.time + self.step_size
    }
}