#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <ctype.h>
#include <stdint.h>
#include <string.h>
#include <wchar.h>

#define N_MAX (100)
#define P_MAX (100)
#define DP_ARRAY_SIZE (N_MAX * P_MAX / 32 + 1)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))
#define ABSS(a, b) ((a) > (b) ? (a) - (b) : (b) - (a))

int compare_f(const void* a, const void* b) {
    return *((int64_t*)a) < *((int64_t*)b) ? -1 : 1;
}

int compare_f2(const void* a, const void* b) {
    return *((uint64_t*)a) > * ((uint64_t*)b) ? -1 : 1;
}

static size_t comb(const size_t n, const size_t r) {
    size_t result = 1;

    for (size_t i = 0; i < r; i++) {
        result *= n - i;
        result /= i + 1;
    }

    return result;
}

static uint64_t combU64(const uint64_t n, const uint64_t r) {
    uint64_t result = 1;

    for (uint64_t i = 0; i < r; i++) {
        result *= n - i;
        result /= i + 1;
    }

    return result;
}

static uint64_t gcd(uint64_t m, uint64_t n)
{
    uint64_t temp;

    while (m % n != 0) {
        temp = n;
        n = m % n;
        m = temp;
    }

    return n;
}

static size_t gcdZu(size_t m, size_t n) {
    size_t temp;

    while (m % n != 0) {
        temp = n;
        n = m % n;
        m = temp;
    }

    return n;
}

static char S[200001], T[200001];

int main(void) {
    size_t H, W, N, sr, sc;

    scanf("%zu %zu %zu\n%zu %zu\n%s\n%s", &H, &W, &N, &sr, &sc, S, T);

    sr--;
    sc--;

    int32_t leftBoundary = 0;
    int32_t rightBoundary = W - 1;
    int32_t upperBoundary = 0;
    int32_t bottomBoundary = H - 1;

    bool kickOutable = false;

    for (size_t cnt = 0; cnt < N; cnt++) {
        {
            if (T[N - cnt - 1] == 'L' && rightBoundary < (int32_t)(W - 1)) {
                rightBoundary++;
            }

            if (S[N - cnt - 1] == 'R') {
                rightBoundary--;
            }
        }
        {
            if (T[N - cnt - 1] == 'R' && 0 < leftBoundary) {
                leftBoundary--;
            }

            if (S[N - cnt - 1] == 'L') {
                leftBoundary++;
            }
        }

        if (rightBoundary < leftBoundary) {
            break;
        }

        {
            if (T[N - cnt - 1] == 'D' && 0 < upperBoundary) {
                upperBoundary--;
            }

            if (S[N - cnt - 1] == 'U') {
                upperBoundary++;
            }
        }
        {
            if (T[N - cnt - 1] == 'U' && bottomBoundary < (int32_t)(H - 1)) {
                bottomBoundary++;
            }

            if (S[N - cnt - 1] == 'D') {
                bottomBoundary--;
            }
        }

        if (bottomBoundary < upperBoundary) {
            break;
        }
    }

    if (leftBoundary <= (int32_t)sc && (int32_t)sc <= rightBoundary && upperBoundary <= (int32_t)sr && (int32_t)sr <= bottomBoundary) {
        puts("YES");
    }
    else {
        puts("NO");
    }

    return 0;
}