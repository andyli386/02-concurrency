use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

#[allow(dead_code)]
#[derive(PartialEq)]
struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Debug,
{
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

#[allow(dead_code)]
fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error : a.row != b.col"));
    }

    let mut data = vec![T::default(); a.row * b.col];

    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[j + i * b.col] += a.data[i * a.col + k] * b.data[k * b.col + j]
            }
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_multiply() -> Result<()> {
        let a = Matrix::new(
            // [[1,2,3]
            // [4,5,6]]
            vec![1, 2, 3, 4, 5, 6],
            2,
            3,
        );
        let b = Matrix::new(
            // [[1,2]
            // [3,4]
            // [5,6]]
            vec![1, 2, 3, 4, 5, 6],
            3,
            2,
        );
        // [[22,28]
        // [49,64]]
        let ret = multiply(&a, &b)?;
        assert_eq!(ret, Matrix::new(vec![22, 28, 49, 64], 2, 2));
        assert_eq!(format!("{}", ret), "{22 28, 49 64}");
        assert_eq!(format!("{:?}", ret), "Matrix(row=2, col=2, {22 28, 49 64})");
        Ok(())
    }
}
