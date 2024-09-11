use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

#[derive(PartialEq, Debug)]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}
impl<T: Debug> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?}", self.data[i * self.col + j])?;
                if j < self.col - 1 {
                    write!(f, ", ")?;
                }
            }
            if i < self.row - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let mut c = vec![T::default(); a.row * b.col];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix {
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            row: 3,
            col: 3,
        };
        let b = Matrix {
            data: vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            row: 3,
            col: 3,
        };
        let expected = Matrix {
            data: vec![30, 24, 18, 84, 69, 54, 138, 114, 90],
            row: 3,
            col: 3,
        };
        assert_eq!(multiply(&a, &b).unwrap(), expected);
        println!("{}", expected);
    }
}
