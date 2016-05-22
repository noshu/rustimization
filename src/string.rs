use lbfgsb_sys::string as ffi;
#[inline]
pub fn stringfy(task:&mut[i8]){
    unsafe{
           ffi::stringfy_(task.as_mut_ptr());
    }
}