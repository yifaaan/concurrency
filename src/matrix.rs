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
            // c[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];

            // a的行向量
            let mut row_elem = vec![];

            row_elem.extend_from_slice(&a.data[i * a.col..(i + 1) * a.col]);
            // b的列向量
            let mut col_elem = vec![];
            col_elem.extend_from_slice(
                &b.data[j..]
                    .iter()
                    .step_by(b.col)
                    .copied()
                    .collect::<Vec<_>>(),
            );
            c[i * b.col + j] += dot_product(row_elem, col_elem)?;
        }
    }
    Ok(Matrix {
        data: c,
        row: a.row,
        col: b.col,
    })
}

fn dot_product<T>(a: Vec<T>, b: Vec<T>) -> anyhow::Result<T>
where
    T: Default + Copy + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Dot product error: a.len()!= b.len()"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix {
            data: vec![2, 0, 1, 3, 0, 0, 5, 1, 1],
            row: 3,
            col: 3,
        };
        let b = Matrix {
            data: vec![1, 0, 1, 1, 2, 1, 1, 1, 0],
            row: 3,
            col: 3,
        };
        let expected = Matrix {
            data: vec![3, 1, 2, 3, 0, 3, 7, 3, 6],
            row: 3,
            col: 3,
        };
        assert_eq!(multiply(&a, &b).unwrap(), expected);
        println!("{}", expected);
    }
}
