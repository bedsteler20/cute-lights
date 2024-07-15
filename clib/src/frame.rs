use std::{future::Future, pin::Pin};

use tokio::task::JoinSet;

use crate::{light::LightPtr, utils::synchronize};

#[repr(C)]
pub struct FramePtr {
    set: Vec<Pin<Box<Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>>>>,
}

#[no_mangle]
pub extern "C" fn frame_new() -> *mut FramePtr {
    let f = FramePtr { set: vec![] };
    Box::into_raw(Box::new(f))
}

#[no_mangle]
pub extern "C" fn frame_clear(f: *mut FramePtr) {
    unsafe {
        (*f).set.clear();
    }
}

#[no_mangle]
pub extern "C" fn frame_free(f: *mut FramePtr) {
    unsafe {
        let _ = Box::from_raw(f);
    }
}

#[no_mangle]
pub extern "C" fn frame_set_on(f: *mut FramePtr, l: *mut LightPtr, on: bool) {
    unsafe {
        (*f).set.push(Box::pin((*l).inner.set_on(on)));
    }
}

#[no_mangle]
pub extern "C" fn frame_set_color(
    f: *mut FramePtr,
    l: *mut LightPtr,
    red: u8,
    green: u8,
    blue: u8,
) {
    unsafe {
        (*f).set
            .push(Box::pin((*l).inner.set_color(red, green, blue)));
    }
}

#[no_mangle]
pub extern "C" fn frame_set_brightness(f: *mut FramePtr, l: *mut LightPtr, brightness: u8) {
    unsafe {
        (*f).set
            .push(Box::pin((*l).inner.set_brightness(brightness)));
    }
}

#[no_mangle]
pub extern "C" fn frame_run(f: *mut FramePtr) {
    unsafe {
        synchronize(async {
            let mut set = JoinSet::new();
            for fut in (*f).set.iter_mut() {
                set.spawn(async move {
                    let _ = fut.await;
                });
            }
            while let Some(Ok(())) = set.join_next().await {}
        });
    }
}
