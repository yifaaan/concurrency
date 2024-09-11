use concurrency::matrix::Matrix;

fn main() -> anyhow::Result<()> {
    let a = Matrix::new(vec![2, 0, 1, 3, 0, 0, 5, 1, 1], 3, 3);
    let b = Matrix::new(vec![1, 0, 1, 1, 2, 1, 1, 1, 0], 3, 3);
    println!("a * b =\n{}", a * b);
    Ok(())
}
