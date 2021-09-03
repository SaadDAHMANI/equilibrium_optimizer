
use rand::distributions::Uniform;
use rand::distributions::Distribution;

pub fn run_eo() {
    


    
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
