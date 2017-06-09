use std::ffi::{CString, CStr};


pub trait AsPtr<T: ?Sized> {
    fn as_ptr(&self) -> *const T;
}


pub trait AsMutPtr<T: ?Sized> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

//------------------------------------------------------------------------------
// References
//------------------------------------------------------------------------------

impl<'a, U: ?Sized, T: ?Sized + AsPtr<U>> AsPtr<U> for &'a T {
    fn as_ptr(&self) -> *const U {
        T::as_ptr(*self)
    }
}


impl<'a, U: ?Sized, T: ?Sized + AsPtr<U>> AsPtr<U> for &'a mut T {
    fn as_ptr(&self) -> *const U {
        T::as_ptr(*self)
    }
}


impl<'a, U: ?Sized, T: ?Sized + AsMutPtr<U>> AsMutPtr<U> for &'a mut T {
    fn as_mut_ptr(&mut self) -> *mut U {
        T::as_mut_ptr(*self)
    }
}


//------------------------------------------------------------------------------
// Slices
//------------------------------------------------------------------------------

impl<T> AsPtr<T> for [T] {
    fn as_ptr(&self) -> *const T {
        self.as_ptr()
    }
}


impl<T> AsMutPtr<T> for [T] {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
}


//------------------------------------------------------------------------------
// Strings
//------------------------------------------------------------------------------

impl AsPtr<u8> for str {
    fn as_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}


//------------------------------------------------------------------------------
// Boxes
//------------------------------------------------------------------------------

impl<T> AsPtr<T> for Box<T> {
    fn as_ptr(&self) -> *const T {
        &**self as *const T
    }
}


impl<T> AsMutPtr<T> for Box<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        &mut **self as *mut T
    }
}


//------------------------------------------------------------------------------
// Vec
//------------------------------------------------------------------------------

impl<T> AsPtr<T> for Vec<T> {
    fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }
}


impl<T> AsMutPtr<T> for Vec<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_slice().as_mut_ptr()
    }
}


//------------------------------------------------------------------------------
// CString
//------------------------------------------------------------------------------

impl AsPtr<i8> for CString {
    fn as_ptr(&self) -> *const i8 {
        (&**self).as_ptr()
    }
}


//------------------------------------------------------------------------------
// CStr
//------------------------------------------------------------------------------

impl AsPtr<i8> for CStr {
    fn as_ptr(&self) -> *const i8 {
        CStr::as_ptr(self)
    }
}
