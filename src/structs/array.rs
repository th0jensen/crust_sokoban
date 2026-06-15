use core::mem::size_of;
use core::ptr::{copy, null_mut, read, write};

use crate::ffi::libc::{free, realloc};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Array<T> {
    pub items: *mut T,
    pub count: usize,
    pub capacity: usize,
}

impl<T> Array<T> {
    pub unsafe fn new() -> Self {
        Self {
            items: null_mut(),
            count: 0,
            capacity: 0,
        }
    }

    pub unsafe fn destroy(array: *mut Self) {
        if array.is_null() {
            return;
        }

        if !(*array).items.is_null() {
            free((*array).items);
            (*array).items = null_mut();
        }

        (*array).count = 0;
        (*array).capacity = 0;
    }
}

pub unsafe fn push<T>(arr: *mut Array<T>, item: T) {
    if (*arr).count >= (*arr).capacity {
        if (*arr).capacity == 0 {
            (*arr).capacity = 256
        } else {
            (*arr).capacity *= 2
        }
        (*arr).items = realloc((*arr).items, size_of::<T>() * (*arr).capacity);
    }

    if (*arr).count > 0 {
        copy((*arr).items, (*arr).items.add(1), (*arr).count);
    }

    write((*arr).items, item);
    (*arr).count += 1
}

pub unsafe fn append<T>(arr: *mut Array<T>, item: T) {
    if (*arr).count >= (*arr).capacity {
        if (*arr).capacity == 0 {
            (*arr).capacity = 256
        } else {
            (*arr).capacity *= 2
        }
        (*arr).items = realloc((*arr).items, size_of::<T>() * (*arr).capacity);
    }
    write((*arr).items.add((*arr).count), item);
    (*arr).count += 1
}

pub unsafe fn pop<T>(arr: *mut Array<T>) -> T {
    (*arr).count -= 1;
    let item = read((*arr).items.add((*arr).count));
    item
}
