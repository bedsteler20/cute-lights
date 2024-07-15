
use cute_lights::Light;

use crate::utils::synchronize;

#[repr(C)]
pub struct LightPtr {
    pub inner: Box<dyn Light>,
}

#[no_mangle]
pub extern "C" fn light_set_on(l: *mut LightPtr, on: bool) -> bool {
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
pub extern "C" fn light_set_color(l: *mut LightPtr, red: u8, green: u8, blue: u8) -> bool {
    unsafe {
        if let Err(e) = synchronize((*l).inner.set_color(red, green, blue)) {
            eprintln!("Error setting light color: {:?}", e);
            false
        } else {
            true
        }
    }
}

#[no_mangle]
pub extern "C" fn light_set_brightness(l: *mut LightPtr, brightness: u8) -> bool {
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
pub extern "C" fn light_get_brightness(l: *mut LightPtr) -> u8 {
    unsafe { (*l).inner.brightness() }
}

#[no_mangle]
pub extern "C" fn light_get_red(l: *mut LightPtr) -> u8 {
    unsafe { (*l).inner.red() }
}

#[no_mangle]
pub extern "C" fn light_get_green(l: *mut LightPtr) -> u8 {
    unsafe { (*l).inner.green() }
}

#[no_mangle]
pub extern "C" fn light_get_blue(l: *mut LightPtr) -> u8 {
    unsafe { (*l).inner.blue() }
}

#[no_mangle]
pub extern "C" fn light_get_is_on(l: *mut LightPtr) -> bool {
    unsafe { (*l).inner.is_on() }
}

#[no_mangle]
pub extern "C" fn light_get_name(l: *mut LightPtr) -> *mut std::os::raw::c_char {
    unsafe {
        let name = (*l).inner.name();
        std::ffi::CString::new(name).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn light_get_id(l: *mut LightPtr) -> *mut std::os::raw::c_char {
    unsafe {
        let id = (*l).inner.id();
        std::ffi::CString::new(id).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn light_get_supports_color(l: *mut LightPtr) -> bool {
    unsafe { (*l).inner.supports_color() }
}

#[no_mangle]
pub extern "C" fn light_free(l: *mut LightPtr) {
    unsafe {
        let _ = Box::from_raw(l);
    }
}
