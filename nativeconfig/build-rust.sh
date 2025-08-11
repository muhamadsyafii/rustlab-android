#!/bin/bash

#
# Created by Muhamad Syafii
# 11/8/2025 - muhamadsyafii4@gmail.com
# Copyright (c) 2025.
# All Rights Reserved
#

#!/bin/bash

# Stop script jika terjadi error
set -e

# Masuk ke direktori Rust
cd rust

# Compile Rust untuk semua target Android dan keluarkan .so ke jniLibs
cargo ndk \
  -t armeabi-v7a \
  -t arm64-v8a \
  -t x86 \
  -t x86_64 \
  -o ../src/main/jniLibs \
  build --release

echo "✅ Build selesai. File .so sudah berada di src/main/jniLibs/"
