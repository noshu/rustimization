use cg_sys::cg as ffi;
#[inline]
pub fn step(n: i32,
            x: &mut [f64],
            f: f64,
            g: &[f64],
            d: &mut [f64],
            gold: &mut [f64],
            iprint: &[i32],
            w: &mut [f64],
            eps: f64,
            iflag: &mut i32,
            irest: i32,
            method: i32,
            finish: i32) {
    unsafe {
        ffi::cgfam_(&n,
                    x.as_mut_ptr(),
                    &f,
                    g.as_ptr(),
                    d.as_mut_ptr(),
                    gold.as_mut_ptr(),
                    iprint.as_ptr(),
                    &eps,
                    w.as_mut_ptr(),
                    iflag,
                    &irest,
                    &method,
                    &finish)
    }
}
