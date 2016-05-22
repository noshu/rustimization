# rustimization
A rust optimization library which includes **L-BFGS-B** and **Conjugate Gradient** algorithm.

##Documentation
The simplest way to use these optimization algorithm is to use the Funcmin class.
```rust
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
```
Output
```
[-4.000000000000021]
```
here Funcmin constructor takes four parameter first one is initial estimation x second and third one is the function f and
the gradient g of the function respectively and forth one is the alogorithm you want to use. Currently two algorithms 
avialabel "cg" and "lbfgsb"
if you want more parameter tuning you can use the clases of the algorithm such as for Lbbfgsb_minimizer clas
###Example
```rust
let f = |x:&Vec<f64>|{ (x[0]+4.0).powf(2.0)};
    let g = |x:&Vec<f64>|{vec![2.0*(x[0]+4.0)]};
    let mut x = vec![40.0f64];
    {
    //creating lbfgsb object. here it takes three parameter
    let mut fmin = Lbfgsb::new(&mut x,&f,&g);
    //seting upper and lower bound first parameter is the index and second one is value
    fmin.set_upper_bound(0,100.0);
    fmin.set_lower_bound(0,10.0);
    //set verbosity. higher value is more verbosity. the value is -1<= to <=101
    fmin.set_verbosity(101);
    //start the algorithm
    fmin.minimize();
    }
    println!("{:?}",x);
```
Output
```
RUNNING THE L-BFGS-B CODE

           * * *

Machine precision = 2.220D-16
 N =            1     M =            5

 L =  1.0000D+01

X0 =  4.0000D+01

 U =  1.0000D+02

At X0         0 variables are exactly at the bounds

At iterate    0    f=  1.93600D+03    |proj g|=  3.00000D+01


ITERATION     1

---------------- CAUCHY entered-------------------
 There are            1   breakpoints 

Piece      1 --f1, f2 at start point  -7.7440D+03  7.7440D+03
Distance to the next break point =   3.4091D-01
Distance to the stationary point =   1.0000D+00
 Variable             1   is fixed.
Cauchy X =  
      1.0000D+01

---------------- exit CAUCHY----------------------

           0  variables are free at GCP            1
 LINE SEARCH           0  times; norm of step =    30.000000000000000     

At iterate    1    f=  1.96000D+02    |proj g|=  0.00000D+00

 X =  1.0000D+01

 G =  2.8000D+01

           * * *

Tit   = total number of iterations
Tnf   = total number of function evaluations
Tnint = total number of segments explored during Cauchy searches
Skip  = number of BFGS updates skipped
Nact  = number of active bounds at final generalized Cauchy point
Projg = norm of the final projected gradient
F     = final function value

           * * *

   N    Tit     Tnf  Tnint  Skip  Nact     Projg        F
    1      1      2      1     0     1   0.000D+00   1.960D+02

 X =  1.0000D+01
  F =   196.00000000000000     

CONVERGENCE: NORM_OF_PROJECTED_GRADIENT_<=_PGTOL            

 Cauchy                time 1.570E-04 seconds.
 Subspace minimization time 0.000E+00 seconds.
 Line search           time 1.800E-05 seconds.

 Total User time 9.330E-04 seconds.

convergence!
```
##Requirements
to use this library you must have **gfortran** installed in your pc
* for **windows** use fortran compiler provided by [mingw](http://www.mingw.org/) or [TDM-GCC](http://tdm-gcc.tdragon.net/)
* for **linux** youcan use the package manager to install gfortran
* for Mac os you can install it form [here](http://hpc.sourceforge.net/) or [here](http://sourceforge.net/projects/hpc/files/hpc/g95/gfortran-mlion.tar.gz)

The orginal ***L-BFGS-B* fortran subroutine is distributed under BSD-3 license. To know more about the condition to use these fortran routine please go [here](http://users.iems.northwestern.edu/~nocedal/lbfgsb.html)
and To know more about the condition to use the Conjugate Gradient Fortran routine please go [here](http://users.iems.northwestern.edu/~nocedal/lbfgsb.html) 

##References
1. R. H. Byrd, P. Lu and J. Nocedal. [A Limited Memory Algorithm for Bound Constrained Optimization](http://www.ece.northwestern.edu/~nocedal/PSfiles/limited.ps.gz), (1995), SIAM Journal on Scientific and Statistical Computing , 16, 5, pp. 1190-1208.
2. C. Zhu, R. H. Byrd and J. Nocedal. [L-BFGS-B: Algorithm 778: L-BFGS-B, FORTRAN routines for large scale bound constrained optimization](http://www.ece.northwestern.edu/~nocedal/PSfiles/lbfgsb.ps.gz) (1997), ACM Transactions on Mathematical Software, Vol 23, Num. 4, pp. 550 - 560.
3. J.L. Morales and J. Nocedal. [L-BFGS-B: Remark on Algorithm 778: L-BFGS-B, FORTRAN routines for large scale bound constrained optimization](http://www.ece.northwestern.edu/~morales/PSfiles/acm-remark.pdf) (2011), to appear in ACM Transactions on Mathematical Software.
4. J. C. Gilbert and J. Nocedal. Global Convergence Properties of Conjugate Gradient Methods for Optimization, (1992) SIAM J. on Optimization, 2, 1.


