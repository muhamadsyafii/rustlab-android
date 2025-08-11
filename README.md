# Rust JNI Integration for Android

This repository demonstrates how to integrate **Rust** with an Android **monolithic Kotlin project** using **JNI**.  
The Rust library stores sensitive constants like **Base URL** and **Bearer Token**, making it harder to reverse-engineer compared to storing them in plain Kotlin code.

---

## 📦 Features
- High-performance and memory-safe native code using Rust
- Store sensitive constants securely in native code
- Easy integration with existing Android monolith projects
- Supports Linux and Windows build environments

---

## 👨‍💻 Maintainers

This project is maintained by:
* [Muhamad Syafii](https://linkedin.com/in/muhamadsyafii4)
---

## Pre-requisites ##
* Android Studio Meerkat | 2024.3.1 Patch 1
* Minimum SDK 24
* Maximum SDK 35
* NDK Version 26.1.10909125

---

## 📂 Project Structure
```
dev.syafii.rustlab/
.
├── app/                              # Main Android application module
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/dev/syafii/rustlab/
│   │   │   │   └── MainActivity.kt   # Android entry point
│   │   │   ├── res/                  # Resources (layouts, drawables, strings, etc.)
│   │   │   └── AndroidManifest.xml
│   │   ├── androidTest/              # Android UI tests
│   │   └── test/                     # Unit tests
│   ├── build.gradle.kts              # Gradle config for the app module
│   ├── proguard-rules.pro
│   └── consumer-rules.pro
│
├── nativeconfig/                      # Additional Android module (optional)
├── src/
│   ├── main/java/dev/syafii/nativeconfig/
│   │   └── NativeConfig.kt         # Kotlin bridge to Rust
│   └── AndroidManifest.xml
│
├── rust/                            # Folder Rust di dalam nativeconfig
│   ├── src/
│   │   ├── target/
│   │   └── lib.rs                   # Rust library dengan JNI functions
│   ├── Cargo.toml                   # Rust package configuration
│   └── Cargo.lock
│
├── build-rust.sh                    # Script build Rust → .so untuk Android
└── build.gradle.kts

│
├── jniLibs/                         # Prebuilt native libraries for Android
│   ├── arm64-v8a/
│   │   └── librust.so
│   ├── armeabi-v7a/
│   │   └── librust.so
│   ├── x86/
│   │   └── librust.so
│   └── x86_64/
│       └── librust.so
│
├── gradle/
│   └── wrapper/
│       ├── gradle-wrapper.jar
│       └── gradle-wrapper.properties
├── build.gradle.kts                   # Root Gradle configuration
├── settings.gradle.kts
└── .gitignore

```

## 🛠️ Installation Guide

### 1. Install Rust
#### **Linux (Ubuntu/Debian/Mint)**
```bash
sudo apt update
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

or u can checkout from url this : https://rustup.rs/
```

#### **Windows**
- Download installer from: [https://rustup.rs/](https://rustup.rs/)
- Run the installer and follow the setup instructions.

#### **Check Rust & Cargo Versions**
```bash
rustc --version
cargo --version
```

#### **Check Rust & Cargo Versions**
```bash
cargo install cargo-ndk
```
---
## 📜 Example Rust Code (rust/src/lib.rs)
```bash
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


#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_getBaseUrl(
    env: JNIEnv,
    _: JClass
) -> jstring {
    let url = BASE_URL.lock().unwrap();
    env.new_string(url.as_str()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_dev_syafii_rustlab_nativeconfig_NativeConfig_getPreferenceName(
    env: JNIEnv,
    _: JClass
) -> jstring {
    let url = PREFERENCE_NAME.lock().unwrap();
    env.new_string(url.as_str()).unwrap().into_raw()
}
```

---

## 📜 Example Cargo (rust/Cargo.toml)
```bash
[package]
name = "nativeconfig"
version = "0.1.0"
edition = "2021"

[lib]
name = "nativeconfig"
crate-type = ["cdylib"]

[dependencies]
jni = "0.21"
once_cell = "1.19"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"
```
---

## 📱 Android Kotlin Integration

```bash
object NativeConfig {
  init {
    System.loadLibrary("nativeconfig")
  }

  external fun getBaseUrl(): String
  external fun clearBaseUrl()
  external fun getPreferenceName() : String

  external fun setToken(token: String)
  external fun getToken(): String
  external fun clearToken()
}
```

---

## ⚙️ Build the Rust Library for Android
```bash
Open your terminal on Android Studio
cd nativeconfig/

Before that please make sure SDK/NDK for rust this to export home like this.
export ANDROID_NDK_HOME=/youtpath/Sdk/ndk/26.1.10909125

please make sure about permission file build-rust.sh
chmod +x build-rust.sh

and now you can build rust using command this.
./build-rust.sh

```

---

## Usage in Kotlin
```bash
val baseUrl = NativeConfig.getBaseUrl()
val prefName = NativeConfig.getPreferenceName()
val token = NativeConfig.getToken()

Log.d("RustJNI", "Base URL: $baseUrl")
Log.d("RustJNI", "Bearer Token: $token")
Log.d("RustJNI", "Preference Name: $prefName")
```

---

## 🔒 Security Comparison: C++ vs Rust

| Aspect | C++ | Rust |
|--------|------|------|
| **Memory Safety** | Relies on manual memory management, prone to issues like buffer overflows, dangling pointers, and use-after-free. | Enforced at compile-time through the *borrow checker*, preventing memory corruption and common safety issues. |
| **Concurrency Safety** | Concurrency is possible but prone to race conditions if not carefully managed. | Prevents data races at compile-time with strict ownership and borrowing rules. |
| **Error Handling** | Exceptions and error codes; requires discipline to handle all cases. | No exceptions; uses `Result` and `Option` types to enforce explicit error handling. |
| **Null Safety** | `nullptr` can cause runtime crashes if dereferenced. | No null references; uses `Option<T>` to explicitly handle nullable values. |
| **Security by Default** | Developer must actively avoid unsafe patterns. | Safe by default; `unsafe` blocks must be explicitly declared and justified. |

> **Conclusion:**  
> Rust’s strict compile-time checks and safe-by-default philosophy make it generally more secure than C++ for preventing memory-related vulnerabilities and concurrency issues.

---
## ⚠️ Important: Update .gitignore for Rust Builds
When integrating Rust, add the following to your .gitignore:
```bash
# =====================
# === Rust (Cargo) ===
# =====================
**/target/
**/Cargo.lock
*.so
*.dll
*.dylib
```
#### Why is this important?
- 📦 Bloating Repo: target/ can grow to hundreds of MB.
- 🔓 Leaking Source Code: .rs files could be pushed accidentally.
- 🐌 Slow Clones: Binary files (.so, .dll, .dylib) make the repo heavy.
- ⚠️ Merge Conflicts: Cargo.lock changes frequently.

✅ Adding these keeps your repository clean, lightweight, and secure.

