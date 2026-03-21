#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define N 4000
#define W 64
#define WORDS ((N + W - 1) / W)
#define MOD 998244353u

typedef struct {
    uint64_t w[WORDS];
} Bitset;

static void bitset_clear(Bitset *a) {
    memset(a->w, 0, sizeof(a->w));
}

static int bitset_is_zero(const Bitset *a) {
    for (int i = 0; i < WORDS; i++) if (a->w[i]) return 0;
    return 1;
}

static int bitset_highest1(const Bitset *a) {
    for (int wi = WORDS - 1; wi >= 0; wi--) {
        uint64_t x = a->w[wi];
        if (x) {
            int b = 63 - __builtin_clzll(x);
            int idx = wi * 64 + b;
            if (idx >= N) idx = N - 1; /* defensive; inputs beyond N are UB per spec */
            return idx;
        }
    }
    return -1;
}

static int bitset_getbit(const Bitset *a, int idx) {
    if (idx < 0 || idx >= N) return 0;
    return (int)((a->w[idx / 64] >> (idx % 64)) & 1ULL);
}

static void bitset_xor_shifted(Bitset *dst, const Bitset *src, int sh) {
    /* dst ^= (src << sh), only within [0, N-1] */
    if (sh < 0) return; /* not expected by spec */
    int word_shift = sh / 64;
    int bit_shift = sh % 64;

    if (word_shift >= WORDS) return;

    if (bit_shift == 0) {
        for (int i = WORDS - 1; i >= word_shift; i--) {
            dst->w[i] ^= src->w[i - word_shift];
        }
    } else {
        for (int i = WORDS - 1; i >= word_shift; i--) {
            uint64_t lo = src->w[i - word_shift] << bit_shift;
            uint64_t hi = 0;
            if (i - word_shift - 1 >= 0) {
                hi = src->w[i - word_shift - 1] >> (64 - bit_shift);
            }
            dst->w[i] ^= (lo | hi);
        }
    }

    /* mask off bits >= N in the top word */
    int extra = WORDS * 64 - N;
    if (extra > 0) {
        dst->w[WORDS - 1] &= (~0ULL >> extra);
    }
}

static void bitset_swap(Bitset *a, Bitset *b) {
    Bitset tmp = *a;
    *a = *b;
    *b = tmp;
}

static void parse_poly(Bitset *out, const char *s) {
    /* s length t, leftmost is highest degree; set bit k from right */
    bitset_clear(out);
    int t = (int)strlen(s);
    for (int pos = 0; pos < t; pos++) {
        char c = s[t - 1 - pos];
        if (c == '1') {
            int idx = pos;
            if (idx >= 0 && idx < N) {
                out->w[idx / 64] |= (1ULL << (idx % 64));
            } else {
                /* out-of-bounds behavior is undefined by spec; do nothing */
            }
        }
    }
}

int main(void) {
    int m;
    if (scanf("%d", &m) != 1) return 0;

    char A_str[N + 5];
    if (scanf("%4004s", A_str) != 1) return 0;

    Bitset A_bits;
    parse_poly(&A_bits, A_str);

    /* Reduction phase: D is not explicitly initialized from A per spec. */
    Bitset D;
    bitset_clear(&D);

    for (int t = 0; t < m; t++) {
        char B_str[N + 5];
        if (scanf("%4004s", B_str) != 1) return 0;

        Bitset B;
        parse_poly(&B, B_str);

        while (!bitset_is_zero(&B)) {
            int i = bitset_highest1(&D);
            int j = bitset_highest1(&B);

            /* If D is zero, i == -1, so i < j and swap will occur. */
            if (i < j) {
                bitset_swap(&D, &B);
                int tmp = i; i = j; j = tmp;
            }

            /* Now i >= j and j >= 0 */
            bitset_xor_shifted(&D, &B, i - j);
        }
    }

    int hiD = bitset_highest1(&D);
    int L = (hiD >= 0) ? (hiD + 1) : 0;

    int lenA = (int)strlen(A_str);
    int degA = lenA - 1;

    /* Precompute powers of two mod MOD up to lenA (inclusive is safe). */
    uint32_t *pow2 = (uint32_t*)malloc((size_t)(lenA + 1) * sizeof(uint32_t));
    if (!pow2) return 0;
    pow2[0] = 1;
    for (int i = 1; i <= lenA; i++) {
        pow2[i] = (uint32_t)((uint64_t)pow2[i - 1] * 2u % MOD);
    }

    Bitset R;
    bitset_clear(&R);
    uint32_t ans = 0;
    int broke = 0;

    for (int i = degA; i >= 0; i--) {
        int x = bitset_getbit(&A_bits, i);
        int y = bitset_getbit(&R, i);

        /* Condition i >= L-1: note when L==0, L-1 == -1, always true. */
        if (i >= (L - 1) && x == 1) {
            int e = i - (L - 1);
            /* e is >= 0 here */
            if (e >= 0 && e <= lenA) {
                ans += pow2[e];
                if (ans >= MOD) ans -= MOD;
            } else {
                /* If e exceeds precomputed range, spec doesn't define; keep narrow:
                   do nothing (inputs are within N, but A length could be up to N). */
            }
        }

        if (x != y) {
            if (i >= (L - 1)) {
                int sh = i - (L - 1);
                bitset_xor_shifted(&R, &D, sh);
                continue;
            } else {
                if (x > y) {
                    ans += 1;
                    if (ans >= MOD) ans -= MOD;
                }
                broke = 1;
                break;
            }
        }
    }

    if (!broke) {
        ans += 1;
        if (ans >= MOD) ans -= MOD;
    }

    printf("%u\n", ans);

    free(pow2);
    return 0;
}
