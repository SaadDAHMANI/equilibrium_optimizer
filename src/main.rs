
include!("equiibrium_optimizer.rs");
include!("benchmarks.rs");

fn main() {
    println!("Equilibrium Optimizer (EO)");

    let particuls = 10;
    let kmax =1;
    let lb =-100.00;
    let ub = 100.00;
    let dim = 3;

    run_eo(particuls, kmax, lb,ub, dim, &f1);
    
}
