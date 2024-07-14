use std::future::Future;

use tokio::task::JoinSet;

static mut RUNTIME: Option<tokio::runtime::Runtime> = None;

pub fn synchronize<F>(future: F) -> F::Output
where
    F: Future,
{
    unsafe {
        if RUNTIME.is_none() {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            RUNTIME = Some(rt);
        }

        RUNTIME.as_ref().unwrap().block_on(future)
    }
}

pub struct FutureBatch<T> {
    futures: JoinSet<T>,
}

impl<T: std::marker::Send> FutureBatch<T> {
    pub fn new() -> Self {
        Self {
            futures: JoinSet::new(),
        }
    }

    pub fn push<F>(&mut self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send,
        T: 'static,
    {
        self.futures.spawn(future);
    }

    pub async fn run(mut self) -> Vec<T>
    where
        T: std::marker::Send,
        T: 'static,
    {
        let mut results = vec![];

        while let Some(Ok(res)) = self.futures.join_next().await {
            results.push(res);
        }

        results
    }
}
