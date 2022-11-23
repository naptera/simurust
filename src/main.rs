use simurust::{base::*, systems::{sources::*, operators::*}};
#[allow(unused_mut)]
fn main() {
    let mut lin: LinearSource<u64, f64> = LinearSource::from(1.0,2.0);
    let mut pol: PolynomialSource<u64, f64, 2> = PolynomialSource::from([2.0, 1.0]);
    let mut add: AddSystem<u64> = AddSystem::new();
    let mut main_system: MainSystem<u64, f64> = MainSystem::new();
    let lin1 = main_system.add_subsystem( Box::new(lin));
    let pol1 = main_system.add_subsystem(Box::new(pol));
    let add1 = main_system.add_subsystem( Box::new(add));

    main_system.connect(pol1, 0, add1);
    main_system.connect(lin1, 0, add1);
    let t = main_system.run_simulation(10);
    println!("{:?}",main_system.get_outputs(add1));
    println!("{}", t);
}