extern crate rustimization;
use rustimization::minimizer::Funcmin;
#[test]
fn test(){
    let f = |x:&Vec<f64>|{ (x[0]+4.0).powf(2.0)};
    let g = |x:&Vec<f64>|{vec![2.0*(x[0]+4.0)]};
    let mut x = vec![40.0f64];
    {
    let mut fmin = Funcmin::new(&mut x,&f,&g,"cg");
    fmin.minimize();
    }
    println!("{:?}",x);
}