use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
};
const NUM_THREADS: usize = 4;

pub struct MsgInput<T> {
    idx: usize,
    row: Vec<T>,
    col: Vec<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vec<T>, col: Vec<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

#[derive(PartialEq, Debug)]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T: PartialEq + Debug> Matrix<T> {
    pub fn new(data: Vec<T>, row: usize, col: usize) -> Self {
        Self { data, row, col }
    }
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

impl<T> Mul for Matrix<T>
where
    T: Debug + Copy + Default + Send + 'static + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multipy error")
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Debug + Copy + Default + Send + 'static + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            // main线程创建channel
            let (sx, rx) = mpsc::channel::<Msg<T>>();
            // 生成线程做点积运算
            std::thread::spawn(move || {
                // 接收从main线程发来的任务
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value: value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            sx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    // oneshot channel的接收端
    let mut receivers = Vec::with_capacity(matrix_len);
    let mut c = vec![T::default(); matrix_len];
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

            // c[i * b.col + j] += dot_product(row_elem, col_elem)?;
            let (sx, rx) = oneshot::channel();
            let idx: usize = i * b.col + j;
            let input = MsgInput::new(idx, row_elem, col_elem);
            let msg = Msg::new(input, sx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }
    // 处理结果
    for rx in receivers {
        let output = rx.recv()?;
        c[output.idx] = output.value;
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
        assert_eq!(a * b, expected);
        println!("{}", expected);
    }
}
