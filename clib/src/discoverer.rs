use crate::{light::LightPtr, utils::synchronize};

#[repr(C)]
pub struct LightDiscovererPtr {
    lights: Box<dyn Iterator<Item = LightPtr>>,
    len: usize,
}

#[no_mangle]
pub extern "C" fn light_discoverer_new() -> *mut LightDiscovererPtr {
    let lights = synchronize(cute_lights::discover_lights());
    let mut lights_c = Vec::new();
    for l in lights {
        lights_c.push(LightPtr { inner: l });
    }

    let ld = LightDiscovererPtr {
        len: lights_c.len(),
        lights: Box::new(lights_c.into_iter()),
    };
    Box::into_raw(Box::new(ld))
}

#[no_mangle]
pub extern "C" fn light_discoverer_next(ld: *mut LightDiscovererPtr) -> *mut LightPtr {
    unsafe {
        match (*ld).lights.next() {
            Some(l) => Box::into_raw(Box::new(l)),
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn light_discoverer_free(ld: *mut LightDiscovererPtr) {
    unsafe {
        let _ = Box::from_raw(ld);
    }
}
