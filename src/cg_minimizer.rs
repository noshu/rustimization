use libc::{c_double, c_int};
use cg::step;
pub struct CG<'a> {
    n: c_int,
    x: &'a mut Vec<c_double>,
    d: Vec<c_double>,
    gold: Vec<c_double>,
    f: &'a Fn(&Vec<c_double>) -> c_double,
    g: &'a Fn(&Vec<c_double>) -> Vec<c_double>,
    eps: c_double,
    w: Vec<c_double>,
    iflag: c_int,
    irest: c_int,
    iprint: Vec<c_int>,
    method: c_int,
    finish: c_int,
    max_iter: u32,
}
impl<'a> CG<'a> {
    // constructor requres three mendatory parameter which is the initial solution, function and the gradient function
    pub fn new(xvec: &'a mut Vec<c_double>,
               func: &'a Fn(&Vec<c_double>) -> c_double,
               gd: &'a Fn(&Vec<c_double>) -> Vec<c_double>)
               -> Self {
        let len = xvec.len() as i32;
        // creating CG struct
        CG {
            n: len,
            x: xvec,
            d: vec![0.0f64;len as usize],
            f: func,
            g: gd,
            eps: 1.0e-7,
            w: vec![0.0f64;len as usize],
            gold: vec![0.0f64;len as usize],
            iprint: vec![1, 3],
            iflag: 0,
            irest: 0,
            method: 1,
            max_iter: 10000,
            finish: 0,
        }
    }
    // this function will start the optimization algorithm
    pub fn minimize(&mut self) {
        let func = self.f;
        let grad = self.g;
        let mut fval = func(self.x);
        let mut gval = grad(self.x);
        let icall = 0;
        // start of the loop
        loop {
            // callign the fortran routine
            step(self.n,
                 &mut self.x,
                 fval,
                 &gval,
                 &mut self.d,
                 &mut self.gold,
                 &self.iprint,
                 &mut self.w,
                 self.eps,
                 &mut self.iflag,
                 self.irest,
                 self.method,
                 self.finish);
            if icall > self.max_iter {
                break;
            }
            match self.iflag {
                // geting the function and gradient value
                1 => {
                    fval = func(self.x);
                    gval = grad(self.x);
                }
                // termination check
                2 => {
                    self.finish = 1;
                    let tlev = self.eps * (1.0 + fval.abs());
                    for l in gval.iter().take(self.x.len()) {
                        if *l > tlev {
                            self.finish = 0;
                            break;
                        }
                    }
                }
                _ => break,
            }
        }
    }
    // this function returns the solution after minimization
    pub fn get_x(&self) -> Vec<c_double> {
        self.x.clone()
    }
    // set max iteration
    pub fn max_iteration(&mut self, i: u32) {
        self.max_iter = i;
    }
    // set restart
    pub fn set_restart(&mut self, b: bool) {
        self.irest = b as i32;
    }
    // set verbosity
    // vec[0] < 0 : NO OUTPUT IS GENERATED
    // vec[0] = 0 : OUTPUT ONLY AT FIRST AND LAST ITERATION
    // vec[0] > 0 : OUTPUT EVERY IPRINT(1) ITERATIONS
    // vec[1]     : SPECIFIES THE TYPE OF OUTPUT GENERATED;
    // vec[1] = 0 : NO ADDITIONAL INFORMATION PRINTED
    // vec[1] = 1 : INITIAL X AND GRADIENT VECTORS PRINTED
    // vec[1] = 2 : X VECTOR PRINTED EVERY ITERATION
    // vec[1] = 3 : X VECTOR AND GRADIENT VECTOR PRINTED
    pub fn set_verbosity(&mut self, v: Vec<i32>) {
        self.iprint = v;
    }
    // set tolernace
    pub fn set_tolerance(&mut self, t: f64) {
        self.eps = t;
    }
}
