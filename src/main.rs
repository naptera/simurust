use simurust::{*, systems::{sources::*, operators::*}};
#[allow(unused_mut)]
fn main() {
    let mut sink1: f64 = 0.0;
    let mut lin: PolynomialSource<f64, 2> = PolynomialSource::new("Linear", &mut sink1);
    let mut sink2: f64 = 0.0;
    let mut pol = PolynomialSource::from("Polynomial", &mut sink2, [1.0, 2.0, 3.0, 4.0], 0.0, 0.1);
    let mut sink3: f64 = 0.0;
    let mut add: AddSystem<f64, 2> = AddSystem::new("Add", [lin.output, pol.output], &mut sink3);
    let t = run_simulation([&mut lin, &mut pol, &mut add], 0.0, 10.0);
    println!("{}", t);
    print!("{}", add);
}