#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <assert.h>

int main(void)
{
    uint32_t A, B, C;
    uint32_t cnt = 0;
    scanf("%" PRIu32 "%" PRIu32 "%" PRIu32, &A, &B, &C);
    while (A != B || B != C) {
        if (A > B && A > C) {
            B++; C++;
        } else if (B > A && B > C) {
            A++; C++;
        } else if (C > A && C > B) {
            A++; B++;
        } else if (A == B) {
            C += 2;
        } else if (B == C) {
            A += 2;
        } else if (C == A) {
            B += 2;
        } else {
            assert(0);
        }
        cnt++;
    }
    printf("%" PRIu32 "\n", cnt);
    return 0;
}

