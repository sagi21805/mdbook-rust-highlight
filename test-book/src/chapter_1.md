# Chapter 1
```hlrs

fn test() {

}

fn test2() {
    let x = 4;
    x.max_size();
    self.test();
}

```

```hlrs
macro_rules! impl_common_address_functions {
    ($struct_name:ident) => {
#[allow(non_snake_case)]
mod ${concat(__impl_for_, $struct_name)} {
    use super::*;
    use core::ptr::Alignment;
    impl $struct_name {
        pub const unsafe fn new_unchecked(address: usize) -> Self {
            Self(address)
        }
        pub const fn as_usize(&self) -> usize {
            self.0
        }
        pub const unsafe fn as_mut_ptr<T>(&self) -> *mut T {
            self.0 as *mut T
        }
        pub const fn as_ptr<T>(&self) -> *const T {
            self.0 as *const T
        }
        pub const fn is_aligned(&self, alignment: Alignment) -> bool {
            self.0 & (alignment.as_usize() - 1) == 0
        }
        pub const fn align_up(mut self, alignment: Alignment) -> Self {
            self.0 = (self.0 + (alignment.as_usize() - 1)) & !(alignment.as_usize() - 1);
            self
        }
        pub const fn align_down(mut self, alignment: Alignment) -> Self {
            self.0 &= !(alignment.as_usize() - 1);
            self
        }
        pub const fn alignment(&self) -> Alignment {
            unsafe { Alignment::new_unchecked(1 << self.0.trailing_zeros()) }
        }
    }
}
    };
}
```


```hlrs,fp=main.rs,icon=@https://icons.veryicon.com/png/o/business/vscode-program-item-icon/assembly-7.png


pub const async unsafe extern "C" fn function(a: b, mut c: D) -> Vec<TestType> {}
```
