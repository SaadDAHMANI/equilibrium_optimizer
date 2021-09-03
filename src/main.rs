
include!("equiibrium_optimizer.rs");
include!("benchmarks.rs");

use std::time::Instant;

fn main() {
    println!("Equilibrium Optimizer (EO) is run ..... ");
    
    let particuls = 30;
    let kmax = 500;
    let lb =-100.00;
    let ub = 100.00;
    let dim = 30;

    let chronos = Instant::now();

    let (fbest, bestsol, convergcrv) = run_eo(particuls, kmax, lb,ub, dim, &f2);

    let duration = chronos.elapsed();

    println!("best fitness : {}", fbest);
    //println!("best solution : {:?}", bestsol);
    println!("best fitness CV {:?}", convergcrv[convergcrv.len()-1]);

    println!("End computation in : {:?}", duration);

}
