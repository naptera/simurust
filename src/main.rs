use simurust::{*, systems::{sources::*, operators::*}};
fn main() {
    let mut lin: Polynomial<f64, 2> = Polynomial::new();
    let mut pol = Polynomial::from([1.0, 2.0, 3.0, 4.0], 0.0, 0.1);
    let mut add: Adder<f64, 2> = Adder::new([&lin.output, &pol.output]);
    let mut neg: Negator<f64> = Negator::new(&lin.output);
    let mut add2: Adder<f64, 2> = Adder::new([&lin.output, &neg.output]);
    let t = run_simulation([&mut lin, &mut pol, &mut add, &mut neg, &mut add2], 0.0, 10.0);
    println!("{}", t);
    print!("{}", add);
    print!("{}", add2);
}