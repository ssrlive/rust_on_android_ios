use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

pub fn inner_rust_greeting(to: &str) -> String {
    "Rust community: Hello ".to_owned() + to
}

/// # Safety
#[no_mangle]
pub unsafe extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = CStr::from_ptr(to);
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new(inner_rust_greeting(recipient)).unwrap().into_raw()
}

/// # Safety
#[no_mangle]
pub unsafe extern fn rust_greeting_free(s: *mut c_char) {
    if s.is_null() { return }
    let _ = CString::from_raw(s);
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString, JObject};
    use self::jni::sys::{jstring};
    use std::os::raw::c_void;

    #[no_mangle]
    pub unsafe extern fn Java_com_krupitskas_pong_RustBindings_greeting(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = rust_greeting(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let output = env.new_string(CStr::from_ptr(world).to_str().unwrap()).expect("Couldn't create java string!");
        rust_greeting_free(world);

        output.into_inner()
    }

    mod fractal;
    mod graphic;

    #[no_mangle]
    pub unsafe extern fn Java_com_krupitskas_pong_RustBindings_renderFractal(env: JNIEnv, _: JClass, bmp: JObject) {
        let mut info = graphic::AndroidBitmapInfo::new();
        let raw_env = env.get_native_interface();

        let bmp = bmp.into_inner();

        // Read bitmap info
        graphic::bitmap_get_info(raw_env, bmp, &mut info);
        let mut pixels = 0 as *mut c_void;

        // Lock pixel for draw
        graphic::bitmap_lock_pixels(raw_env, bmp, &mut pixels);

        let pixels =
            std::slice::from_raw_parts_mut(pixels as *mut u8, (info.stride * info.height) as usize);

        fractal::render(pixels, info.width as u32, info.height as u32);
        graphic::bitmap_unlock_pixels(raw_env, bmp);
    }    
}