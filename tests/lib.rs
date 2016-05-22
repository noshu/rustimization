extern crate rustimization;
use rustimization::lbfgsb_minimizer::Lbfgsb;
#[test]
fn test(){
    let f = |x:&Vec<f64>|{ (x[0]+4.0).powf(2.0)};
    let g = |x:&Vec<f64>|{vec![2.0*(x[0]+4.0)]};
    let mut x = vec![40.0f64];
    {
    let mut fmin = Lbfgsb::new(&mut x,&f,&g);
    fmin.set_upper_bound(0,100.0);
    fmin.set_lower_bound(0,10.0);
    fmin.set_verbosity(101);
    fmin.minimize();
    }
    println!("{:?}",x);
}