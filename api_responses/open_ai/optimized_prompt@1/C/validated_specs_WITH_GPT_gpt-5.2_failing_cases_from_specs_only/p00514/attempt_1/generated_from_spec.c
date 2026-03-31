#include <stdio.h>
#include <stdint.h>

#define BASE 1000000000U
#define MAX_BLOCKS 8667 /* fixed capacity in base 1e9 blocks (~77,338 digits) */

typedef struct {
    uint32_t d[MAX_BLOCKS]; /* little-endian blocks */
    int len;                /* number of used blocks, len>=1 */
} Big;

/* Normalize length (remove leading zero blocks) while keeping at least 1 block. */
static void big_norm(Big *a) {
    while (a->len > 1 && a->d[a->len - 1] == 0) a->len--;
}

/* Set Big to small nonnegative value v (v fits in uint32_t). */
static void big_set_u32(Big *a, uint32_t v) {
    a->d[0] = v;
    for (int i = 1; i < MAX_BLOCKS; i++) a->d[i] = 0;
    a->len = 1;
    big_norm(a);
}

/* Multiply Big by small positive integer m (exact, within capacity). */
static void big_mul_u64(Big *a, uint64_t m) {
    uint64_t carry = 0;
    for (int i = 0; i < a->len; i++) {
        uint64_t cur = (uint64_t)a->d[i] * m + carry;
        a->d[i] = (uint32_t)(cur % BASE);
        carry = cur / BASE;
    }
    while (carry != 0) {
        if (a->len >= MAX_BLOCKS) {
            /* Undefined behavior per spec if capacity exceeded; we just stop growing. */
            break;
        }
        a->d[a->len++] = (uint32_t)(carry % BASE);
        carry /= BASE;
    }
    big_norm(a);
}

/* Divide Big by small positive integer v, assuming exact divisibility.
   Returns remainder (should be 0 for exact division). */
static uint32_t big_div_u32(Big *a, uint32_t v) {
    uint64_t rem = 0;
    for (int i = a->len - 1; i >= 0; i--) {
        uint64_t cur = a->d[i] + rem * BASE;
        a->d[i] = (uint32_t)(cur / v);
        rem = cur % v;
    }
    big_norm(a);
    return (uint32_t)rem;
}

static void big_print(const Big *a) {
    /* Print most significant block without leading zeros, then remaining with 9 digits. */
    int i = a->len - 1;
    printf("%u", a->d[i]);
    for (i = i - 1; i >= 0; i--) {
        printf("%09u", a->d[i]);
    }
    putchar('\n');
}

int main(void) {
    int64_t n, m, r;
    if (scanf("%" SCNd64 " %" SCNd64 " %" SCNd64, &n, &m, &r) != 3) {
        /* No explicit error handling required by spec. */
        return 0;
    }

    /* Compute r' = r - n*m using mathematical integer arithmetic.
       We use int64_t; if overflow occurs, behavior is outside preconditions (undefined). */
    int64_t rp = r - n * m;

    if (rp < 0) {
        puts("0");
        return 0;
    }
    if (rp == 0) {
        puts("1");
        return 0;
    }

    /* Binomial coefficient case: output C(n+rp-1, n-1).
       Preconditions: n>=1 and values within reasonable ranges. */
    int64_t N = n + rp - 1;     /* top */
    int64_t K = n - 1;          /* choose */
    int64_t R = rp;             /* alternative choose is rp */

    /* Use symmetry to minimize steps: C(N,K) where K = min(K, N-K) = min(n-1, rp). */
    int64_t k = K;
    if (R < k) k = R;

    Big ans;
    big_set_u32(&ans, 1);

    /* Multiplicative formula:
       ans = product_{i=1..k} (N - k + i) / i
       We perform exact multiply then exact divide each step. */
    for (int64_t i = 1; i <= k; i++) {
        int64_t mult = (N - k + i);
        /* Preconditions imply mult fits in internal types and division is exact. */
        big_mul_u64(&ans, (uint64_t)mult);
        (void)big_div_u32(&ans, (uint32_t)i);
    }

    big_print(&ans);
    return 0;
}
