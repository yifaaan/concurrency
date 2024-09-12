use dashmap::DashMap;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|e| *e += 1)
            .or_insert(0);
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|e| *e -= 1)
            .or_insert(0);
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{")?;

        for entry in self.data.iter() {
            write!(f, "{}: {},", entry.key(), entry.value())?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}
