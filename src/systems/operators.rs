use crate::base::*;
#[derive(Clone, Debug)]
pub struct AddSystem<T: Time> {
    inputs: Vec<usize>,
    output: usize,
    time: T,
    step_size: T,
}
impl<T: Time> AddSystem<T> {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            output: 0,
            time: T::zero(),
            step_size: T::default_step_size(),
        }
    }
}
impl<T: Time, V: Value> SimSystem<T,V> for AddSystem<T> {
    fn next_step(&mut self, stream: &mut Vec<V>){
        self.time += self.step_size;
        let mut sum = V::zero();
        for i in &self.inputs {
            sum += stream[*i];
        }
        stream[self.output] = sum;
    }
    fn get_next_time(&self) -> T {
        self.time + self.step_size
    }
    fn get_dim(&self) -> usize {
        1
    }
    fn add_input(&mut self, input: usize) {
        self.inputs.push(input);
    }
    fn set_outputs(&mut self, output_start: usize) {
        self.output = output_start;
    }
    fn get_output_start(&self) -> usize {
        self.output
    }
}