
include!("equiibrium_optimizer.rs");
include!("benchmarks.rs");

fn main() {
    println!("Equilibrium Optimizer (EO)");

    let particuls = 30;
    let kmax =500;
    let lb =-100.00;
    let ub = 100.00;
    let dim = 12;

   let (fbest, bestsol, convergcrv) = run_eo(particuls, kmax, lb,ub, dim, &f1);

    println!("best fitness : {}", fbest);
    println!("best solution : {:?}", bestsol);
    println!("best fitness CV {:?}", convergcrv[convergcrv.len()-1]);

}
