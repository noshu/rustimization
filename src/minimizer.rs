use cg_minimizer::CG;
use lbfgsb_minimizer::Lbfgsb;
pub struct Funcmin<'a> {
    x: &'a mut Vec<f64>,
    f: &'a Fn(&Vec<f64>) -> f64,
    g: &'a Fn(&Vec<f64>) -> Vec<f64>,
    tol: f64,
    verbose: bool,
    method: &'a str,
    max_iter: u32,
}
impl<'a> Funcmin<'a> {
    // constructor requres three mendatory parameter which is the initial solution, function and the gradient function
    pub fn new(xvec: &'a mut Vec<f64>,
               func: &'a Fn(&Vec<f64>) -> f64,
               gd: &'a Fn(&Vec<f64>) -> Vec<f64>,
               m: &'a str)
               -> Self {
        // creating Funcmin struct
        Funcmin {
            x: xvec,
            f: func,
            g: gd,
            tol: 1.0e-7,
            max_iter: 10000,
            verbose: false,
            method: m,
        }
    }
    // this function will start the optimization algorithm
    pub fn minimize(&mut self) {
        let ver = if self.verbose {
            0
        } else {
            1
        };
        {
            if self.method == "lbfgsb" {
                let mut minf = Lbfgsb::new(&mut self.x, self.f, self.g);
                minf.set_verbosity(ver);
                minf.set_tolerance(self.tol);
                minf.max_iteration(self.max_iter);
                minf.minimize();
            } else if self.method == "cg" {
                let mut minf = CG::new(&mut self.x, self.f, self.g);
                minf.set_verbosity(vec![ver, 0]);
                minf.set_tolerance(self.tol);
                minf.max_iteration(self.max_iter);
                minf.minimize();
            } else {
                println!("wrong method provided");
            }

        }
    }
    // this function returns the solution after minimization
    pub fn get_x(&self) -> Vec<f64> {
        self.x.clone()
    }
    pub fn set_tolerance(&mut self, t: f64) {
        self.tol = t;
    }
    // set max iteration
    pub fn max_iteration(&mut self, i: u32) {
        self.max_iter = i;
    }
    pub fn set_verbosity(&mut self, b: bool) {
        self.verbose = b;
    }
}
