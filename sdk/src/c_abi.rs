use std::{future::Future, pin::Pin};

use tokio::task::JoinSet;

use crate::{CuteResult, utils::future::synchronize, integrations::Light};

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct light {
    pub inner: Box<dyn Light>,
}

#[no_mangle]
pub extern "C" fn light_set_on(l: *mut light, on: bool) -> bool {
    unsafe {
        if let Err(e) = synchronize((*l).inner.set_on(on)) {
            eprintln!("Error setting light on: {:?}", e);
            false
        } else {
            true
        }
    }
}

#[no_mangle]
pub extern "C" fn light_set_color(l: *mut light, h: i64, s: i64, b: i64) -> bool {
    unsafe {
        if let Err(e) = synchronize((*l).inner.set_color(h, s, b)) {
            eprintln!("Error setting light color: {:?}", e);
            false
        } else {
            true
        }
    }
}

#[no_mangle]
pub extern "C" fn light_set_brightness(l: *mut light, brightness: i64) -> bool {
    unsafe {
        if let Err(e) = synchronize((*l).inner.set_brightness(brightness)) {
            eprintln!("Error setting light brightness: {:?}", e);
            false
        } else {
            true
        }
    }
}

#[no_mangle]
pub extern "C" fn light_get_brightness(l: *mut light) -> i64 {
    unsafe { (*l).inner.brightness() }
}

#[no_mangle]
pub extern "C" fn light_get_hue(l: *mut light) -> i64 {
    unsafe { (*l).inner.hue() }
}

#[no_mangle]
pub extern "C" fn light_get_saturation(l: *mut light) -> i64 {
    unsafe { (*l).inner.saturation() }
}

#[no_mangle]
pub extern "C" fn light_get_is_on(l: *mut light) -> bool {
    unsafe { (*l).inner.is_on() }
}

#[no_mangle]
pub extern "C" fn light_get_name(l: *mut light) -> *mut std::os::raw::c_char {
    unsafe {
        let name = (*l).inner.name();
        std::ffi::CString::new(name).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn light_get_id(l: *mut light) -> *mut std::os::raw::c_char {
    unsafe {
        let id = (*l).inner.id();
        std::ffi::CString::new(id).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn light_get_supports_color(l: *mut light) -> bool {
    unsafe { (*l).inner.supports_color() }
}

#[no_mangle]
pub extern "C" fn light_free(l: *mut light) {
    unsafe {
        let _ = Box::from_raw(l);
    }
}

// =================================================================================================

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct light_discoverer {
    pub lights: Box<dyn Iterator<Item = light>>,
    len: usize,
}

#[no_mangle]
pub extern "C" fn light_discoverer_new() -> *mut light_discoverer {
    let lights = synchronize(crate::discover::discover_lights());
    let mut lights_c = Vec::new();
    for l in lights {
        lights_c.push(light { inner: l });
    }

    let ld = light_discoverer {
        len: lights_c.len(),
        lights: Box::new(lights_c.into_iter()),
    };
    Box::into_raw(Box::new(ld))
}

#[no_mangle]
pub extern "C" fn light_discoverer_next(ld: *mut light_discoverer) -> *mut light {
    unsafe {
        match (*ld).lights.next() {
            Some(l) => Box::into_raw(Box::new(l)),
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn light_discoverer_free(ld: *mut light_discoverer) {
    unsafe {
        let _ = Box::from_raw(ld);
    }
}


// =================================================================================================

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct frame {
    set: Vec<Pin<Box<Pin<Box<dyn Future<Output = CuteResult<()>> + Send>>>>>,
}

#[no_mangle]
pub extern "C" fn frame_new() -> *mut frame {
    let f = frame { set: vec![] };
    Box::into_raw(Box::new(f))
}

#[no_mangle]
pub extern "C" fn frame_clear(f: *mut frame) {
    unsafe {
        (*f).set.clear();
    }
}

#[no_mangle]
pub extern "C" fn frame_free(f: *mut frame) {
    unsafe {
        let _ = Box::from_raw(f);
    }
}

#[no_mangle]
pub extern "C" fn frame_set_on(f: *mut frame, l: *mut light, on: bool) {
    unsafe {
        (*f).set.push(Box::pin((*l).inner.set_on(on)));
    }
}

#[no_mangle]
pub extern "C" fn frame_set_color(f: *mut frame, l: *mut light, h: i64, s: i64, b: i64) {
    unsafe {
        (*f).set.push(Box::pin((*l).inner.set_color(h, s, b)));
    }
}

#[no_mangle]
pub extern "C" fn frame_set_brightness(f: *mut frame, l: *mut light, brightness: i64) {
    unsafe {
        (*f).set
            .push(Box::pin((*l).inner.set_brightness(brightness)));
    }
}

#[no_mangle]
pub extern "C" fn frame_run(f: *mut frame) {
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
