#!/bin/bash
set -euo pipefail

# Android용 hancat-ffi 빌드 스크립트
# 각 ABI별 .so 파일을 jniLibs 구조로 생성합니다.
#
# 사전 준비:
#   cargo install cargo-ndk
#   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
#   ANDROID_NDK_HOME 환경변수 설정
#
# 사용법:
#   ./scripts/build-android.sh          # 전체 빌드
#   ./scripts/build-android.sh grade-a  # grade-a feature로 빌드

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
OUT_DIR="$PROJECT_DIR/dist/android/jniLibs"

TARGETS=(
    aarch64-linux-android     # arm64-v8a
    armv7-linux-androideabi   # armeabi-v7a
    x86_64-linux-android      # x86_64
    i686-linux-android        # x86
)

# Android API 레벨 (최소 지원 버전)
API_LEVEL=21

FEATURE_FLAGS=""
if [ "${1:-}" != "" ]; then
    FEATURE_FLAGS="--features $1"
fi

if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    echo "오류: ANDROID_NDK_HOME 환경변수를 설정하세요."
    echo "  예: export ANDROID_NDK_HOME=\$HOME/Android/Sdk/ndk/27.0.12077973"
    exit 1
fi

echo "==> Rust 타겟 설치 확인"
for target in "${TARGETS[@]}"; do
    rustup target add "$target" 2>/dev/null || true
done

echo "==> cargo-ndk 확인"
if ! command -v cargo-ndk &>/dev/null; then
    echo "오류: cargo-ndk가 설치되어 있지 않습니다."
    echo "  설치: cargo install cargo-ndk"
    exit 1
fi

echo "==> 빌드 시작 (API level $API_LEVEL)"
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cargo ndk \
    --manifest-path "$PROJECT_DIR/Cargo.toml" \
    --target aarch64-linux-android \
    --target armv7-linux-androideabi \
    --target x86_64-linux-android \
    --target i686-linux-android \
    --platform "$API_LEVEL" \
    --output-dir "$OUT_DIR" \
    -- build --release $FEATURE_FLAGS

# 헤더 파일 복사
cp "$PROJECT_DIR/include/hancat.h" "$PROJECT_DIR/dist/android/"

echo "==> 완료: $OUT_DIR"
echo "    구조:"
find "$OUT_DIR" -name "*.so" | sort | while read -r f; do
    echo "    $(echo "$f" | sed "s|$PROJECT_DIR/||")"
done
