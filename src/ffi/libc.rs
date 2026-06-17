use core::ffi::c_void;
extern "C" {
    pub fn exit(sig: i32) -> !;
    pub fn rand() -> i32;
    pub fn srand(seed: u32);
    pub fn time(t: *mut isize) -> isize;
    pub fn strcmp(a: *const i8, b: *const i8) -> i32;
    pub fn ceil(x: f64) -> f64;

    #[link_name = "malloc"]
    fn c_malloc(size: usize) -> *mut c_void;
    #[link_name = "realloc"]
    fn c_realloc(ptr: *const c_void, size: usize) -> *const c_void;
    #[link_name = "free"]
    fn c_free(ptr: *const c_void);
}

#[macro_export]
macro_rules! c_concat {
    ($($s:literal),+ $(,)?) => {
        concat!($($s),*, "\0").as_ptr() as *const i8
    };
}

#[macro_export]
macro_rules! printf {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            extern "C" {
                fn printf(fmt: *const i8, ...) -> i32;
            }
            printf($crate::c_concat!($fmt) $(, $arg)*)
        }
    }};
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            extern "C" {
                fn printf(fmt: *const i8, ...) -> i32;
            }
            printf($crate::c_concat!($fmt, "\n") $(, $arg)*)
        }
    }};
}

pub unsafe fn malloc<T>(size: usize) -> *mut T {
    c_malloc(size) as *mut T
}

pub unsafe fn realloc<T>(ptr: *const T, size: usize) -> *mut T {
    c_realloc(ptr as *mut c_void, size) as *mut T
}

pub unsafe fn free<T>(ptr: *const T) {
    c_free(ptr as *mut c_void)
}
