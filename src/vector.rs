use anyhow::anyhow;
use anyhow::Result;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref, Mul},
};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }

    // pub fn len(&self) -> usize {
    //     self.data.len()
    // }
    //
    // pub fn iter(&self) -> std::slice::Iter<T> {
    //     self.data.iter()
    // }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.len() != b.len() {
        return Err(anyhow!("Matrix multiply error : a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
