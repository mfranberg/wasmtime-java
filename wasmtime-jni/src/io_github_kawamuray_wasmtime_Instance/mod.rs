// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniInstanceImpl;
use jni::descriptors::Desc;
use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;

macro_rules! wrap_error {
    ($env:expr, $body:expr, $default:expr) => {
        match $body {
            Ok(v) => v,
            Err(e) => {
                if let Err(err) = $env.throw(e) {
                    $env.exception_describe().ok();
                    panic!("error in throwing exception: {}", err);
                }
                $default
            }
        }
    };
}

trait JniInstance<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_get_func(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        name: JString,
    ) -> Result<jlong, Self::Error>;
    fn native_get_memory(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        name: JString,
    ) -> Result<jlong, Self::Error>;
    fn new_instance(
        env: &JNIEnv,
        clazz: JClass,
        store_ptr: jlong,
        module_ptr: jlong,
        externs: jobjectArray,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(
        env,
        JniInstanceImpl::dispose(&env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_nativeGetFunc__JLjava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
    name: JString,
) -> jlong {
    wrap_error!(
        env,
        JniInstanceImpl::native_get_func(&env, this, store_ptr, name),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_nativeGetMemory__JLjava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
    name: JString,
) -> jlong {
    wrap_error!(
        env,
        JniInstanceImpl::native_get_memory(&env, this, store_ptr, name),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_newInstance__JJ_3Lio_github_kawamuray_wasmtime_Extern_2(
    env: JNIEnv,
    clazz: JClass,
    store_ptr: jlong,
    module_ptr: jlong,
    externs: jobjectArray,
) -> jlong {
    wrap_error!(
        env,
        JniInstanceImpl::new_instance(&env, clazz, store_ptr, module_ptr, externs),
        Default::default()
    )
}
