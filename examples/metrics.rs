use concurrency::metrics::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone())
    }

    for _ in 0..M {
        request_worker(metrics.clone())
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("{:?}", metrics.snapshot());
    }
    Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) {
    std::thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx));
    });
}

fn request_worker(metrics: Metrics) {
    std::thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));

        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page));
    });
}
