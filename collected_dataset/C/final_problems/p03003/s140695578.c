#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <ctype.h>
#include <stdint.h>
#include <string.h>
#include <wchar.h>
#include <math.h>

#define N_MAX (100)
#define P_MAX (100)
#define DP_ARRAY_SIZE (N_MAX * P_MAX / 32 + 1)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))
#define ABSS(a, b) ((a) > (b) ? (a) - (b) : (b) - (a))

int compare_sz_desc(const void* a, const void* b) {
    return *((size_t*)a) > * ((size_t*)b) ? -1 : 1;
}

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

static uint64_t gcdU64(uint64_t m, uint64_t n)
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

#define MOD_VAL (UINT64_C(1000000007))

static uint64_t dp[2000][2000];
static uint64_t sum[2000][2000];

int main(void) {
    size_t N, M;
    uint64_t S[2000], T[2000];

    scanf("%zu %zu\n", &N, &M);
    for (size_t i = 0; i < N; i++) {
        scanf("%"PRIu64, &(S[i]));
        scanf(i < N - 1 ? " " : "\n");
    }
    for (size_t i = 0; i < M; i++) {
        scanf("%"PRIu64, &(T[i]));
        if (i < M - 1) {
            scanf(" ");
        }
    }

    for (size_t sI = 0; sI < N; sI++) {
        for (size_t tI = 0; tI < M; tI++) {
            if (S[sI] == T[tI]) {
                if (0 < sI && 0 < tI) {
                    dp[sI][tI] = sum[sI - 1][tI - 1] + 1;
                }
                else {
                    dp[sI][tI] = 1;
                }
            }
            else {
                dp[sI][tI] = 0;
            }

            sum[sI][tI] = 0;

            if (0 < sI) {
                sum[sI][tI] += sum[sI - 1][tI];
            }
            if (0 < tI) {
                sum[sI][tI] += sum[sI][tI - 1];
            }
            if (0 < sI && 0 < tI) {
                sum[sI][tI] -= sum[sI - 1][tI - 1];
            }

            sum[sI][tI] += dp[sI][tI];
            sum[sI][tI] %= MOD_VAL;
        }
    }

    printf("%"PRIu64, sum[N - 1][M - 1] + 1);

    return 0;
}
