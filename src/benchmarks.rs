

pub fn f1(x : &Vec<f64>)-> f64 {
    let mut sum : f64 = 0.0;

    for value in x.iter(){
    sum += value.powi(2);
    }  
    sum
}

fn f2(x: &Vec<f64>)-> f64 {
    //o=sum(abs(x))+prod(abs(x));
    let mut sum : f64 = 0.0;
    let mut prod : f64 =1.0;

     for value in x.iter(){
         sum += value.abs();
         prod *=value.abs();
     }  
       sum + prod    
}



