# HanCat FFI

[hancat-core](https://github.com/tossicat/hancat-core)의 C FFI 바인딩입니다.
게임 엔진(Unreal, Unity, Godot 등)이나 C/C++ 프로젝트에서
한국어 조사(토시) 변환과 용언 활용 기능을 사용할 수 있게 해줍니다.

## 빌드

```bash
cargo build --release
```

빌드하면 다음 파일들이 생성됩니다:

- `include/hancat.h` — C 헤더 파일
- `target/release/libhancat_ffi.so` (Linux)
- `target/release/libhancat_ffi.dylib` (macOS)
- `target/release/hancat_ffi.dll` (Windows)

## C 언어에서 사용법

```c
#include "hancat.h"
#include <stdio.h>

int main() {
    // 토시(조사) 변환
    char* r1 = hancat_modify("{포션, 을} 획득했습니다!");
    if (r1) {
        printf("%s\n", r1);  // "포션을 획득했습니다!"
        hancat_free(r1);
    }

    // 용언 활용
    char* r2 = hancat_modify("여기서 {쉬다, 세요}.");
    if (r2) {
        printf("%s\n", r2);  // "여기서 쉬세요."
        hancat_free(r2);
    }

    // 토시 + 용언 통합 처리
    char* r3 = hancat_modify("{철수, 이} {밥, 을} {먹다, 었습니다}.");
    if (r3) {
        printf("%s\n", r3);  // "철수가 밥을 먹었습니다."
        hancat_free(r3);
    }

    return 0;
}
```

### 컴파일 (Linux)

```bash
gcc -o example example.c -L target/release -lhancat_ffi
```

### 컴파일 (macOS)

```bash
gcc -o example example.c -L target/release -lhancat_ffi
```

## 이 프로젝트의 장점

- **토시(조사) + 용언 활용 통합**: 하나의 함수(`hancat_modify`)로 조사 변환과 용언 활용을 동시에 처리합니다.
- **GitHub 최신 소스 사용**: 기본적으로 [hancat-core](https://github.com/tossicat/hancat-core) GitHub 저장소의 최신 코드를 사용합니다.
- **용언 등급 선택 가능**: 용언 사전 크기를 선택할 수 있습니다.
  ```bash
  # grade-a: 최소 230개 용언
  cargo build --release --features grade-a

  # grade-b: 863개 용언
  cargo build --release --features grade-b
  ```
- **게임에서의 활용**: 한국어 게임에서 아이템명이나 캐릭터명에 따라 조사를 자동으로 붙이고, 용언 활용까지 처리할 수 있습니다.

```c
// 게임 아이템 획득 메시지
void show_item_message(const char* item_name) {
    char template[256];
    snprintf(template, sizeof(template), "{%s, 을} 획득했습니다!", item_name);

    char* message = hancat_modify(template);
    if (message) {
        show_ui_text(message);  // 게임 UI에 표시
        hancat_free(message);
    }
}

// show_item_message("포션");   → "포션을 획득했습니다!"
// show_item_message("검");     → "검을 획득했습니다!"
// show_item_message("활");     → "활을 획득했습니다!"
// show_item_message("마나");   → "마나를 획득했습니다!"
```

## API 목록

| 함수 | 설명 |
|------|------|
| `hancat_modify(input)` | 문장 내 `{단어, 접사}` 패턴을 토시/용언 활용으로 변환 |
| `hancat_last_error()` | 마지막 에러 메시지 반환 (해제 불필요) |
| `hancat_free(ptr)` | 반환된 문자열 메모리 해제 |

## 에러 코드

`hancat_modify()`는 처리 실패 시 에러 코드를 해당 위치에 삽입합니다:

| 코드 | 설명 |
|------|------|
| `{E01}` | 닫는 중괄호(`}`) 없음 |
| `{E02}` | 쉼표(`,`) 없음 |
| `{E03}` | 빈 단어 |
| `{E04}` | 빈 접사 |
| `{E10}` | 용언 사전에 없는 단어 |
| `{E11}` | 해당 용언에 맞는 어미 없음 |
| `{E12}` | 토시(조사) 미존재 |

## 주의사항

- 모든 문자열은 **UTF-8 인코딩**이어야 합니다.
- `hancat_modify()`가 반환한 문자열은 **반드시 `hancat_free()`로 해제**해야 합니다.
- NULL 포인터를 입력하면 NULL을 반환합니다.

## CI

[![CI](https://github.com/tossicat/hancat-ffi/actions/workflows/ci.yml/badge.svg)](https://github.com/tossicat/hancat-ffi/actions/workflows/ci.yml)

`main` 브랜치에 push하거나 PR을 올리면 GitHub Actions를 통해 자동으로 테스트와 빌드가 실행됩니다.

- **Linux**, **macOS**, **Windows** 3개 플랫폼에서 병렬 실행
- `cargo test` — 단위 테스트 자동 실행
- `cargo build --release` — 릴리스 빌드 및 산출물 artifact 업로드

결과는 [Actions 탭](https://github.com/tossicat/hancat-ffi/actions)에서 확인할 수 있습니다.

## 의존성

- [hancat-core](https://github.com/tossicat/hancat-core) (GitHub, `source-github` feature 기본 활성화)

## 활용 방법

빌드된 라이브러리(`libhancat_ffi.so`/`.dylib`/`.dll`)와 헤더 파일(`include/hancat.h`)을 프로젝트에 복사하여 사용합니다.

### C/C++

헤더를 포함하고 라이브러리를 링크합니다.

```bash
gcc -o myapp myapp.c -I include -L target/release -lhancat_ffi
```

실행 시 동적 라이브러리 경로를 지정합니다.

```bash
# Linux
LD_LIBRARY_PATH=target/release ./myapp

# macOS
DYLD_LIBRARY_PATH=target/release ./myapp
```

### Unreal Engine (C++)

1. 빌드된 라이브러리를 `Plugins/HanCat/Binaries/` 에 복사합니다.
2. `hancat.h`를 `Plugins/HanCat/Source/` 에 복사합니다.
3. `.Build.cs`에서 라이브러리를 링크합니다.

```cpp
#include "hancat.h"

FString GetItemMessage(const FString& ItemName) {
    FString Template = FString::Printf(TEXT("{%s, 을} 획득했습니다!"), *ItemName);
    char* Result = hancat_modify(TCHAR_TO_UTF8(*Template));
    if (Result) {
        FString Message = UTF8_TO_TCHAR(Result);
        hancat_free(Result);
        return Message;
    }
    return TEXT("");
}
```

### Unity (C#)

```csharp
using System.Runtime.InteropServices;

public static class HanCat {
    [DllImport("hancat_ffi")]
    private static extern IntPtr hancat_modify(string input);

    [DllImport("hancat_ffi")]
    private static extern void hancat_free(IntPtr ptr);

    public static string Modify(string input) {
        IntPtr ptr = hancat_modify(input);
        if (ptr == IntPtr.Zero) return null;
        string result = Marshal.PtrToStringUTF8(ptr);
        hancat_free(ptr);
        return result;
    }
}

// 사용 예시
// string msg = HanCat.Modify("{포션, 을} 획득했습니다!");  // "포션을 획득했습니다!"
```

### Godot (GDScript + GDExtension)

GDExtension C API를 통해 바인딩하거나, GDNative를 사용합니다.

```gdscript
# gdextension으로 래핑한 경우
var result = HanCat.modify("{포션, 을} 획득했습니다!")
print(result)  # "포션을 획득했습니다!"
```

## 라이선스

MIT
