use simurust::{*, systems::{sources::*, operators::*}};
fn main() {
    let mut pol = Polynomial::from([1.0, 2.0, 3.0, 4.0], 0.0, 0.1);
    let mut gain: Gain<f64> = Gain::new();
    connect!(gain, gain);
    let t = run_simulation([&mut pol, &mut gain], 0.0, 10.0);
    println!("{}", t);
    print!("{}", gain.get_output());
}