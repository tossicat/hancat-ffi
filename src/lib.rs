//! # HanCat C FFI
//!
//! hancat-core의 C FFI 바인딩입니다.
//! 게임 엔진(Unreal, Unity, Godot 등)이나 C/C++ 프로젝트에서
//! 한국어 조사(토시) 변환과 용언 활용 기능을 사용할 수 있게 해줍니다.
//!
//! ## 사용 예시 (C)
//!
//! ```c
//! #include "hancat.h"
//!
//! char* result = hancat_modify("{포션, 을} 획득했습니다!");
//! if (result) {
//!     printf("%s\n", result);  // "포션을 획득했습니다!"
//!     hancat_free(result);
//! } else {
//!     const char* err = hancat_last_error();
//!     printf("에러: %s\n", err);
//! }
//! ```

#[cfg(feature = "source-github")]
use hancat_git as hancat;

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

thread_local! {
    static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

/// 에러 메시지를 thread-local 변수에 저장
fn set_last_error(msg: &str) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = CString::new(msg).unwrap_or_default();
    });
}

/// 에러 상태를 초기화
fn clear_last_error() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = CString::default();
    });
}

/// ## 마지막 에러 메시지를 반환하는 함수
///
/// 에러가 없으면 빈 문자열을 반환합니다.
/// 반환된 포인터는 다음 hancat 함수 호출 전까지만 유효합니다.
/// `hancat_free()`로 해제하지 마세요.
///
/// ### 사용 예시 (C)
/// ```c
/// char* result = hancat_modify(input);
/// if (!result) {
///     const char* err = hancat_last_error();
///     printf("에러: %s\n", err);
/// }
/// ```
#[no_mangle]
pub extern "C" fn hancat_last_error() -> *const c_char {
    LAST_ERROR.with(|e| e.borrow().as_ptr())
}

/// ## 문장 내의 모든 `{단어, 접사}` 패턴을 처리하여 변환된 문장을 반환하는 함수
///
/// C에서 사용하는 `modify()` 함수입니다.
/// 반환된 문자열은 반드시 `hancat_free()`로 해제해야 합니다.
///
/// - 접사가 어미이고 단어가 용언이면 용언 활용 처리
/// - 그 외에는 토시(조사) 처리
/// - 처리 실패 시 에러 코드(`{E01}`~`{E12}`)를 해당 위치에 삽입합니다.
///
/// ### 매개변수
/// - `input`: 변환할 문장 (UTF-8 인코딩 C 문자열)
///   - 형식: `"{단어, 접사} 문장"`
///
/// ### 반환값
/// - 성공: 변환된 문장
/// - 실패: NULL (`hancat_last_error()`로 원인 조회)
///
/// ### 사용 예시 (C)
/// ```c
/// // 토시(조사) + 용언 활용 통합 처리
/// char* r1 = hancat_modify("{철수, 이} {밥, 을} {먹다, 었습니다}.");
/// // r1: "철수가 밥을 먹었습니다."
/// hancat_free(r1);
///
/// // 토시(조사)만 처리
/// char* r2 = hancat_modify("{철수, 은} {영희, 과} {밥, 를} 먹습니다.");
/// // r2: "철수는 영희와 밥을 먹습니다."
/// hancat_free(r2);
///
/// // 용언만 처리
/// char* r3 = hancat_modify("여기서 {쉬다, 세요}.");
/// // r3: "여기서 쉬세요."
/// hancat_free(r3);
/// ```
#[no_mangle]
pub extern "C" fn hancat_modify(input: *const c_char) -> *mut c_char {
    clear_last_error();

    if input.is_null() {
        set_last_error("NULL 포인터가 전달되었습니다");
        return ptr::null_mut();
    }

    let input_str = match unsafe { CStr::from_ptr(input) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("input: 잘못된 UTF-8 인코딩입니다");
            return ptr::null_mut();
        }
    };

    let result = hancat::modify(input_str);

    match CString::new(result) {
        Ok(c_result) => c_result.into_raw(),
        Err(e) => {
            set_last_error(&format!("결과 문자열 변환 실패: {}", e));
            ptr::null_mut()
        }
    }
}

/// ## hancat 함수들이 반환한 문자열을 해제하는 함수
///
/// `hancat_modify()`가 반환한 문자열을
/// 이 함수로 반드시 해제해야 합니다.
///
/// ### 매개변수
/// - `s`: 해제할 문자열 포인터. NULL이면 무시합니다.
///
/// ### 사용 예시 (C)
/// ```c
/// char* result = hancat_modify("{검, 을} 획득했습니다!");
/// printf("%s\n", result);
/// hancat_free(result);  // 반드시 해제
/// ```
#[no_mangle]
pub extern "C" fn hancat_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    /// CStr 포인터에서 String으로 변환하는 헬퍼
    unsafe fn ptr_to_str(ptr: *const c_char) -> String {
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_owned()
    }

    /// 결과 포인터를 String으로 변환하고 메모리를 해제하는 헬퍼
    unsafe fn take_result(ptr: *mut c_char) -> String {
        assert!(!ptr.is_null());
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap()
            .to_owned();
        hancat_free(ptr);
        s
    }

    // === modify 테스트 ===

    #[test]
    fn modify_토시_기본() {
        let input = CString::new("{철수, 은} {영희, 과} {밥, 를} 먹습니다.").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "철수는 영희와 밥을 먹습니다.");
    }

    #[test]
    fn modify_용언_활용() {
        let input = CString::new("여기서 {쉬다, 세요}.").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "여기서 쉬세요.");
    }

    #[test]
    fn modify_토시_용언_통합() {
        let input = CString::new("{철수, 이} {밥, 을} {먹다, 었습니다}.").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "철수가 밥을 먹었습니다.");
    }

    #[test]
    fn modify_패턴_없는_문장() {
        let input = CString::new("일반 문장입니다.").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "일반 문장입니다.");
    }

    #[test]
    fn modify_null() {
        let result = hancat_modify(ptr::null());
        assert!(result.is_null());
        assert!(unsafe { ptr_to_str(hancat_last_error()) }.contains("NULL"));
    }

    #[test]
    fn modify_빈_문자열() {
        let input = CString::new("").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "");
    }

    #[test]
    fn modify_파싱_에러_닫는_중괄호_없음() {
        let input = CString::new("{철수, 이").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "{E01}");
    }

    #[test]
    fn modify_파싱_에러_쉼표_없음() {
        let input = CString::new("{철수 이}").unwrap();
        let result = hancat_modify(input.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "{E02}");
    }

    // === hancat_free 테스트 ===

    #[test]
    fn free_null_안전() {
        hancat_free(ptr::null_mut()); // 크래시 없어야 함
    }

    // === hancat_last_error 테스트 ===

    #[test]
    fn last_error_성공시_초기화() {
        // 먼저 에러를 발생시킴
        hancat_modify(ptr::null());
        assert!(!unsafe { ptr_to_str(hancat_last_error()) }.is_empty());

        // 성공 호출 후 에러가 초기화되는지 확인
        let input = CString::new("{검, 을} 획득!").unwrap();
        let result = hancat_modify(input.as_ptr());
        if !result.is_null() {
            assert!(unsafe { ptr_to_str(hancat_last_error()) }.is_empty());
            hancat_free(result);
        }
    }
}
