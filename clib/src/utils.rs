use std::future::Future;


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
