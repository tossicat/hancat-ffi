#!/bin/bash
set -euo pipefail

# iOS용 hancat-ffi 빌드 스크립트
# XCFramework (디바이스 + 시뮬레이터)를 생성합니다.
#
# 사전 준비:
#   rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
#
# 사용법:
#   ./scripts/build-ios.sh          # 전체 빌드
#   ./scripts/build-ios.sh grade-a  # grade-a feature로 빌드

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
OUT_DIR="$PROJECT_DIR/dist/ios"

TARGETS=(
    aarch64-apple-ios       # 디바이스
    aarch64-apple-ios-sim   # 시뮬레이터 (Apple Silicon)
    x86_64-apple-ios        # 시뮬레이터 (Intel)
)

FEATURE_FLAGS=""
if [ "${1:-}" != "" ]; then
    FEATURE_FLAGS="--features $1"
fi

echo "==> Rust 타겟 설치 확인"
for target in "${TARGETS[@]}"; do
    rustup target add "$target" 2>/dev/null || true
done

echo "==> 빌드 시작"
for target in "${TARGETS[@]}"; do
    echo "  -> $target"
    cargo build --release --target "$target" $FEATURE_FLAGS --manifest-path "$PROJECT_DIR/Cargo.toml"
done

echo "==> XCFramework 생성"
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

# 시뮬레이터용 fat library (arm64 + x86_64)
mkdir -p "$OUT_DIR/sim-fat"
lipo -create \
    "$PROJECT_DIR/target/aarch64-apple-ios-sim/release/libhancat_ffi.a" \
    "$PROJECT_DIR/target/x86_64-apple-ios/release/libhancat_ffi.a" \
    -output "$OUT_DIR/sim-fat/libhancat_ffi.a"

xcodebuild -create-xcframework \
    -library "$PROJECT_DIR/target/aarch64-apple-ios/release/libhancat_ffi.a" \
    -headers "$PROJECT_DIR/include" \
    -library "$OUT_DIR/sim-fat/libhancat_ffi.a" \
    -headers "$PROJECT_DIR/include" \
    -output "$OUT_DIR/HanCatFFI.xcframework"

rm -rf "$OUT_DIR/sim-fat"

echo "==> 완료: $OUT_DIR/HanCatFFI.xcframework"
