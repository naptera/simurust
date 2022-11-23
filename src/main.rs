use num::{Num, NumCast, Complex, pow::pow};
use std::{ops::AddAssign};


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
    fn get_time(&self) -> T;
    fn get_dim(&self) -> usize;
    fn add_input(&mut self, input: usize);
    fn set_outputs(&mut self, output_start: usize);
    fn get_output_start(&self) -> usize;
}

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

    fn get_time(&self) -> T {
        self.time
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
    fn get_time(&self) -> T {
        self.time
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
    fn get_time(&self) -> T {
        self.time
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

    pub fn run_simulation(&mut self, stop_time: T) {
        // TODO: stop !before! stop_time is reached
        while self.time < stop_time {
            let mut min_time = self.subsystems[0].get_time();
            let mut index: usize = 0;
            for i in 1..self.subsystems.len() {
                if self.subsystems[i].get_time() < min_time {
                    index = i;
                    min_time = self.subsystems[i].get_time();
                }
            }
            self.time = min_time;
            self.subsystems[index].as_mut().next_step(&mut self.stream);
        }
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


fn main() {
    let mut lin: LinearSource<f32, f32> = LinearSource::from(1.0,2.0);
    let mut pol: PolynomialSource<f32, f32, 2> = PolynomialSource::from([2.0, 1.0]);
    let mut add: AddSystem<f32> = AddSystem::new();
    let mut main_system: MainSystem<f32, f32> = MainSystem::new();
    let lin1 = main_system.add_subsystem( Box::new(lin));
    let pol1 = main_system.add_subsystem(Box::new(pol));
    let add1 = main_system.add_subsystem( Box::new(add));

    main_system.connect(pol1, 0, add1);
    main_system.connect(lin1, 0, add1);
    main_system.run_simulation(10.0);
    println!("{:?}",main_system.get_outputs(add1));
}