/* automatically generated by rust-bindgen 0.70.1 */

pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __builtin_va_list;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn Rvprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list);
    pub fn REvprintf(arg1: *const ::std::os::raw::c_char, arg2: va_list);
}