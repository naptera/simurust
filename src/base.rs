use num::{Num, NumCast, Complex};
use std::ops::AddAssign;
pub trait Value: Num + AddAssign + NumCast + Copy {
    fn cast<V: Value>(other: V) -> Self {
        Self::from(other).expect("Some types are not castable between each other")
    }
}
macro_rules! impl_value_trait {
    ($($t:ty),*) => ($(
        impl Value for $t {}
        impl Value for Complex<$t> {}
    )*)
}
impl_value_trait!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

pub trait Time: Value + PartialOrd {
    fn default_step_size() -> Self;
}
impl Time for u32{
    fn default_step_size() -> Self {
        1u32
    }
}
impl Time for u64{
    fn default_step_size() -> Self {
        1u64
    }
}
impl Time for f32{
    fn default_step_size() -> Self {
        0.1f32
    }
}
impl Time for f64{
    fn default_step_size() -> Self {
        0.1f64
    }
}

pub trait SimSystem<T: Time, V: Value> {
    fn next_step(&mut self, stream: &mut Vec<V>);
    fn get_next_time(&self) -> T;
    fn get_dim(&self) -> usize;
    fn add_input(&mut self, input: usize);
    fn set_outputs(&mut self, output_start: usize);
    fn get_output_start(&self) -> usize;
}
pub struct MainSystem<T: Time, V: Value> {
    subsystems: Vec<Box<dyn SimSystem<T,V>>>,
    stream: Vec<V>,
    time: T,
}

impl<T: Time, V: Value> MainSystem<T,V> {
    pub fn new() -> Self {
        Self {
            subsystems: Vec::new(),
            stream: Vec::new(),
            time: T::zero()
        }
    }

    pub fn add_subsystem(&mut self, mut subsystem: Box<dyn SimSystem<T,V>>) -> usize {
        subsystem.set_outputs(self.stream.len());
        self.stream.append(&mut vec![V::zero();subsystem.get_dim()]);
        self.subsystems.push(subsystem);
        self.subsystems.len()-1
    }

    pub fn connect(&mut self, subsys_a: usize, out_a: usize, subsys_b: usize) {
        let output = self.subsystems[subsys_a].get_output_start() + out_a;
        self.subsystems[subsys_b].add_input(output);
    }

    pub fn run_simulation(&mut self, stop_time: T) -> T {
        while self.time <= stop_time {
            let mut min_time = self.subsystems[0].get_next_time();
            let mut index: usize = 0;
            for i in 1..self.subsystems.len() {
                if self.subsystems[i].get_next_time() < min_time {
                    index = i;
                    min_time = self.subsystems[i].get_next_time();
                }
            }
            self.time = min_time;
            self.subsystems[index].as_mut().next_step(&mut self.stream);
        }
        self.time
    }

    pub fn get_outputs(&self, subsys: usize) -> Vec<V> {
        let output_start = self.subsystems[subsys].get_output_start();
        let dim = self.subsystems[subsys].get_dim();
        let mut outputs: Vec<V>= Vec::new(); 
        for i in output_start..output_start+dim {
            outputs.push(self.stream[i]);
        }
        outputs
    }
} 