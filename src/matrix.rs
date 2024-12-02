use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;

#[allow(dead_code)]
#[derive(PartialEq)]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
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
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error : a.row != b.col"));
        // panic!("Matrix multiply error : a.row != b.col");
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("{}", e);
                    };
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];

    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);

            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
        // println!(
        //     "data {:?}, output idx = {:?}, output value = {:?}",
        //     data, output.idx, output.value
        // );
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Multiply error")
    }
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

        let ret = a * b;
        assert_eq!(ret, Matrix::new(vec![22, 28, 49, 64], 2, 2));
        assert_eq!(format!("{}", ret), "{22 28, 49 64}");
        assert_eq!(format!("{:?}", ret), "Matrix(row=2, col=2, {22 28, 49 64})");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b() {
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
            vec![1, 2, 3, 4],
            2,
            2,
        );
        // let _ret = multiply(&a, &b);
        let _ret = a * b;
        // assert!(ret.is_err());
    }
}
