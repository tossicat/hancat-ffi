#include "hancat.h"
#include <stdio.h>

int main() {
    // 1. 토시(조사) 변환
    printf("=== 토시(조사) 변환 ===\n");

    char* r1 = hancat_modify("{포션, 을} 획득했습니다!");
    if (r1) {
        printf("%s\n", r1);
        hancat_free(r1);
    }

    char* r2 = hancat_modify("{마나, 을} 획득했습니다!");
    if (r2) {
        printf("%s\n", r2);
        hancat_free(r2);
    }

    // 2. 문장 내 여러 토시 일괄 변환
    printf("\n=== 여러 토시 일괄 변환 ===\n");

    char* s1 = hancat_modify("{철수, 은} {영희, 과} {밥, 를} 먹습니다.");
    if (s1) {
        printf("%s\n", s1);
        hancat_free(s1);
    }

    char* s2 = hancat_modify("{전사, 이} {검, 으로} {드래곤, 을} 공격합니다.");
    if (s2) {
        printf("%s\n", s2);
        hancat_free(s2);
    }

    // 3. 용언 활용
    printf("\n=== 용언 활용 ===\n");

    char* v1 = hancat_modify("여기서 {쉬다, 세요}.");
    if (v1) {
        printf("%s\n", v1);
        hancat_free(v1);
    }

    // 4. 토시 + 용언 통합
    printf("\n=== 토시 + 용언 통합 ===\n");

    char* m1 = hancat_modify("{철수, 이} {밥, 을} {먹다, 었습니다}.");
    if (m1) {
        printf("%s\n", m1);
        hancat_free(m1);
    }

    return 0;
}
