use crate::base::*;
use num::pow::pow;
#[derive(Clone, Debug)]
pub struct PolynomialSource<T: Time, V: Value, const N: usize> {
    coefficients: [V;N],
    output: usize,
    time: T,
    step_size: T,
}

impl<T: Time, V: Value, const N: usize> PolynomialSource<T,V,N> {
    pub fn new() -> Self {
        Self {
            coefficients: [V::zero();N],
            output: 0,
            time: T::zero(),
            step_size: T::default_step_size(),
        }
    }
    pub fn from(coefficients: [V;N]) -> Self {
        Self {
            coefficients,
            output: 0,
            time: T::zero(),
            step_size: T::default_step_size(),
        }
    }
}

impl<T: Time, V: Value, const N: usize> SimSystem<T,V> for PolynomialSource<T,V,N> {
    fn next_step(&mut self, stream: &mut Vec<V>) {
        let mut output: V = V::zero();
        self.time += self.step_size;
        for i in 0..self.coefficients.len() {
            output += self.coefficients[i]*pow(V::cast(self.time),i);
        }
        stream[self.output] = output;
    }

    fn get_next_time(&self) -> T {
        self.time + self.step_size
    }

    fn get_dim(&self) -> usize {
        1
    }

    fn add_input(&mut self, _input: usize) {}

    fn set_outputs(&mut self, output_start: usize) {
        self.output = output_start;
    }
    fn get_output_start(&self) -> usize {
        self.output
    }
}

#[derive(Clone, Debug)]
pub struct LinearSource<T: Time, V: Value> {
    slope: V,
    offset: V,
    output: usize,
    time: T,
    step_size: T,
}
impl<T: Time, V: Value> LinearSource<T,V> {
    pub fn new() -> Self {
        Self {
            slope: V::one(),
            offset: V::zero(),
            output: 0,
            time: T::zero(),
            step_size: T::default_step_size(),
        }
    }
    pub fn from(slope: V, offset: V) -> Self {
        Self {
            slope,
            offset,
            output: 0,
            time: T::zero(),
            step_size: T::default_step_size(),
        }
    }
}

impl<T,V> SimSystem<T,V> for LinearSource<T,V>
where T: Time, V: Value {
    fn next_step(&mut self, stream: &mut Vec<V>){
        self.time += self.step_size;
        stream[self.output] = self.slope*V::cast(self.time) + self.offset;
    }
    fn get_next_time(&self) -> T {
        self.time + self.step_size
    }
    fn get_dim(&self) -> usize {
        1
    }
    fn add_input(&mut self, _input: usize) {}
    fn set_outputs(&mut self, output_start: usize) {
        self.output = output_start;
    }
    fn get_output_start(&self) -> usize {
        self.output
    }
}