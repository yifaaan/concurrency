use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

fn main() -> anyhow::Result<()> {
    Ok(())
}

#[derive(Debug)]
struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Debug + Copy + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let mut c = Vec::with_capacity(a.row * b.col);
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                c[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data: c,
        row: a.row,
        col: b.col,
    })
}
