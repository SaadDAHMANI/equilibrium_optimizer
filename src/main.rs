// Equilibrium optimizer
// A. Faramarzi, M. Heidarinejad, B. Stephens, S. Mirjalili. Equilibrium optimizer: 
// A novel optimization algorithm. Knowledge-Based Systems (2019). 
// https://doi.org/10.1016/j.knosys.2019.105190
//----------------------------------------------------------------------------------
// Implemented in Rust programming language by :
// Saad Dahmani (sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz)
// https://github.com/SaadDAHMANI/equilibrium_optimizer
//----------------------------------------------------------------------------------


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
