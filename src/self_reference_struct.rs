use std::marker::PhantomPinned;
use std::pin::Pin;

pub struct SelfReferenceStruct {
    numbers: Box<Vec<i32>>,
    sel_a: * const i32,
    sel_b: * const i32,
    _marker: PhantomPinned,
}

impl SelfReferenceStruct {
    pub fn new() -> Self {
        let numbers = Box::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let sel_a = numbers.get(2).unwrap() as * const i32;
        let sel_b = numbers.get(8).unwrap() as * const i32;
        Self {
            numbers,
            sel_a,
            sel_b,
            _marker: PhantomPinned::default(),
        }
    }

    pub fn get_sel_a(&self) -> i32 {
        unsafe { *self.sel_a }
    }

    pub fn get_sel_b(&self) -> i32 {
        unsafe { *self.sel_b }
    }

    pub fn set_sel_a(&mut self, index: usize) {
        self.sel_a = self.numbers.get(index).unwrap() as * const i32;
    }

    pub fn set_sel_b(&mut self, index: usize) {
        self.sel_b = self.numbers.get(index).unwrap() as * const i32;
    }

}