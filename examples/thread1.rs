use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

const NUM_PRODUCERS: usize = 10;

#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}

fn main() -> anyhow::Result<()> {
    let (sx, rx) = mpsc::channel();

    // 创建producers
    for idx in 0..NUM_PRODUCERS {
        let sx = sx.clone();
        thread::spawn(move || {
            produce(idx, sx);
        });
    }

    // 创建consumer
    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))?;
    Ok(())
}

fn produce(idx: usize, sx: Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let value = rand::random::<usize>();
        sx.send(Msg::new(idx, value))?;
        std::thread::sleep(Duration::from_millis(1000));
    }
}
