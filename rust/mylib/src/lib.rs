#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {

    use jni::objects::JClass;
    use jni::sys::jint;
    use jni::JNIEnv;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_krupitskas_pong_MylibBindings_add(
        _: JNIEnv,
        _: JClass,
        a: jint,
        b: jint,
    ) -> jint {
        a + b
    }
}
