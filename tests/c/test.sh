#!/usr/bin/env bash

set -Eeuo pipefail

cmake -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build
cd build
ctest
