
use rand::distributions::Uniform;
use rand::distributions::Distribution;

pub fn run_eo(particles_no : usize, max_iter : usize, lb : f64, ub : f64, dim : usize, fobj : &dyn Fn(&Vec<f64>)->f64) {
    
// Initialize variables 
//Ceq1=zeros(1,dim);   Ceq1_fit=inf; 
//Ceq2=zeros(1,dim);   Ceq2_fit=inf; 
//Ceq3=zeros(1,dim);   Ceq3_fit=inf; 
//Ceq4=zeros(1,dim);   Ceq4_fit=inf;

let mut ceq1 = vec![0.0f64; dim];

let mut ceq2 = vec![0.0f64; dim];

let mut ceq3 = vec![0.0f64; dim];

let mut ceq4 = vec![0.0f64; dim];

let mut ceq1_fit = f64::MAX;

let mut ceq2_fit = f64::MAX;

let mut ceq3_fit = f64::MAX;

let mut ceq4_fit = f64::MAX;


//C=initialization(Particles_no,dim,ub,lb);
let mut c =initialization(particles_no, dim, lb, ub);

// Iter=0; V=1;
let mut iter =0;
let v =1;

//
// a1=2;
// a2=1;
// GP=0.5;
let a1 = 2.0;
let a2 = 1.0;
let gp = 0.5;




}

fn initialization(searchagents_no : usize, dim : usize, lb : f64, ub : f64)-> Vec<Vec<f64>>{
    let mut positions = vec![vec![0.0f64; dim]; searchagents_no];
    let intervall01 = Uniform::from(0.0f64..=1.0f64);
    let mut rng = rand::thread_rng();              
    
    for i in 0..searchagents_no {
         for  j in 0..dim {   
              positions[i][j]= intervall01.sample(&mut rng)*(ub-lb)+lb;                         
         }
    }    
    
    positions
}
