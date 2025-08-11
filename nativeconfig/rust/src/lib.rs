use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Default (konstanta) base URL
static DEFAULT_BASE_URL: &str = "https://reqres.in";
static DEFAULT_TOKEN: &str = "Bearer 12312313123";
static DEFAULT_PREFERENCE_NAME: &str = "dev.syafii.rustlab.pref";

// Global base URL dan token — bisa berubah saat runtime (thread-safe)
static BASE_URL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(DEFAULT_BASE_URL.to_string()));
static TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(Some(DEFAULT_TOKEN.to_string())));
static PREFERENCE_NAME: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(DEFAULT_PREFERENCE_NAME.to_string()));

/// ✅ Get current Base URL
#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_getBaseUrl(
    env: JNIEnv,
    _: JClass
) -> jstring {
    let url = BASE_URL.lock().unwrap();
    env.new_string(url.as_str()).unwrap().into_raw()
}

/// ✅ Reset base URL ke default
#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativecJava_dev_syafii_rustlab_nativeconfig_NativeConfig_clearTokenonfig_NativeConfig_clearBaseUrl(
    _: JNIEnv,
    _: JClass
) {
    let mut url = BASE_URL.lock().unwrap();
    *url = DEFAULT_BASE_URL.to_string();
}

/// ✅ Set token dari Kotlin
#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_setToken(
    mut env: JNIEnv,
    _: JClass,
    token: JString
) {
    let input: String = env.get_string(&token).unwrap().into();
    let mut token_lock = TOKEN.lock().unwrap();
    *token_lock = Some(input);
}

/// ✅ Get token yang tersimpan
#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_getToken(
    env: JNIEnv,
    _: JClass
) -> jstring {
    let token_lock = TOKEN.lock().unwrap();
    let token = token_lock.clone().unwrap_or_else(|| "".to_string());
    env.new_string(token).unwrap().into_raw()
}

/// ✅ Clear token
#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_clearToken(
    _: JNIEnv,
    _: JClass
) {
    let mut token_lock = TOKEN.lock().unwrap();
    *token_lock = None;
}

/// ✅ Get preferenceName
#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_getPreferenceName(
    env: JNIEnv,
    _: JClass
) -> jstring {
    let url = PREFERENCE_NAME.lock().unwrap();
    env.new_string(url.as_str()).unwrap().into_raw()
}