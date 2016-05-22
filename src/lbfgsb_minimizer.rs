use libc::{c_char, c_double,c_int};
use std::ffi::CStr;
use lbfgsb::step;
use string::stringfy;
pub struct Lbfgsb<'a>{
    n:c_int,
    m:c_int,
    x:&'a mut Vec<c_double>,
    l:Vec<c_double>,
    u:Vec<c_double>,
    nbd:Vec<c_int>,
    f:&'a Fn(&Vec<c_double>)->c_double,
    g:&'a Fn(&Vec<c_double>)->Vec<c_double>,
    factr:c_double,
    pgtol:c_double,
    wa:Vec<c_double>,
    iwa:Vec<c_int>,
    task:Vec<c_char>,
    iprint: c_int,
    csave: Vec<c_char>,
    lsave: Vec<c_int>,
    isave:Vec<c_int>,
    dsave:Vec<c_double>,
    max_iter: u32
}
impl<'a> Lbfgsb<'a>{
    //constructor requres three mendatory parameter which is the initial solution, function and the gradient function
    pub fn new(xvec:&'a mut Vec<c_double>,func:&'a Fn(&Vec<c_double>)->c_double,gd:&'a Fn(&Vec<c_double>)->Vec<c_double>)->Self{
        let len = xvec.len() as i32;
        //creating lbfgs struct
        Lbfgsb{n:len,m:5,x:xvec,l:vec![0.0f64;len as usize],u:vec![0.0f64;len as usize],nbd:vec![0;len as usize],f:func,g:gd,
             factr:0.0e0,pgtol:0.0e0,wa:vec![0.0f64;(2*5*len+11*5*5+5*len+8*5) as usize],iwa:vec![0;3*len as usize],task:vec![0;60],iprint:-1,csave:vec![0;60],
             lsave:vec![0,0,0,0],isave:vec![0;44],dsave:vec![0.0f64;29],max_iter:0
        }
    }
    //this function will start the optimization algorithm
    pub fn minimize(&mut self){
        let mut fval = 0.0f64;
        let mut gval = vec![0.0f64;self.x.len()];
        let func = self.f;
        let grad = self.g;
        //converting fortran string "STRAT"
        stringfy(&mut self.task);
       //start of the loop
       loop{
           //callign the fortran routine
           step(self.n,self.m,&mut self.x,&self.l,&self.u,&self.nbd,fval,&gval,self.factr,self.pgtol,&mut self.wa,
                 &mut self.iwa,&mut self.task,self.iprint,&mut self.csave,&mut self.lsave,&mut self.isave,&mut self.dsave);
            //converting to rust string
            let tsk = unsafe{CStr::from_ptr(self.task.as_ptr()).to_string_lossy()};
            if &tsk[0..2]=="FG" {
                fval = func(& self.x);
                gval = grad(& self.x);
            }
            if &tsk[0..5]=="NEW_X" && self.max_iter==0{
                if self.dsave[11]<=1.0e-10*(1.0e0+fval.abs()){
                    println!("THE PROJECTED GRADIENT IS SUFFICIENTLY SMALL");
                    break;
                }
            }
            if self.max_iter>0{
                if self.isave[29]>= self.max_iter as i32 {break;}
            }
            if &tsk[0..4]=="CONV"{
                println!("convergence!");
                break;
            }
            if &tsk[0..5]=="ERROR"{
                println!("error in the input parameters");
            }
            if &tsk[0..8]=="ABNORMAL"{
                println!("ERROR: ABNORMAL TERMINATION");
                break;
             }
        }
    }
    //this function returns the solution after minimization
    pub fn get_x(&self)->Vec<c_double>{
        self.x.clone()
    }
    //this function is used to set lower bounds to a variable
    pub fn set_lower_bound(&mut self,index:usize,value:f64){
        if self.nbd[index]==1 || self.nbd[index]==2{
            println!("variable already has Lower Bound");
        }
        else{
            let temp = self.nbd[index]-1;
            self.nbd[index] = if temp<0{temp*(-1)}else{temp};
            self.l[index] = value;
        }
    }
    //this function is used to set upper bounds to a variable
    pub fn set_upper_bound(&mut self,index:usize,value:f64){
        if self.nbd[index]==3 || self.nbd[index]==2{
            println!("variable already has Lower Bound");
        }
        else{
            self.nbd[index] = 3-self.nbd[index];
            self.u[index] = value;
        }
    }
    //set the verbosity level
    pub fn set_verbosity(&mut self,l:i32){
        self.iprint = l;
    }
    //set termination tolerance
        //1.0e12 for low accuracy
        //1.0e7  for moderate accuracy
        //1.0e1  for extremely high accuracy
    pub fn set_termination_tolerance(&mut self,t:f64){
        self.factr = t;
    }
    //set tolerance of projection gradient
    pub fn set_tolerance(&mut self,t:f64){
        self.pgtol = t;
    }    
    //set max iteration
    pub fn max_iteration(&mut self,i:u32){
        self.max_iter = i;
    }
    //set maximum number of variable metric corrections
    //The range  3 <= m <= 20 is recommended
    pub fn set_matric_correction(&mut self,m:i32){
        self.m = m;
    }
}