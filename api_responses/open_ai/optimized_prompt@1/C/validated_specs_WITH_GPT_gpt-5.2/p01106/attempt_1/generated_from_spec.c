#include <stdio.h>
#include <stdint.h>

int main(void) {
    long long n_in;
    long long i_in, j_in;

    /* Fixed internal storage; spec warns about n > 60 being unsafe/undefined. */
    char out[61 + 1];

    while (1) {
        if (scanf("%lld", &n_in) != 1) break;
        if (n_in <= 0) break;
        if (scanf("%lld %lld", &i_in, &j_in) != 2) break;

        int n = (int)n_in;

        /*
         * Interpret i and j as 1-based; use (i-1) and (j-1) for n-bit decisions.
         * Use unsigned arithmetic to avoid undefined behavior on signed overflow.
         */
        uint64_t a = (uint64_t)(i_in - 1);
        uint64_t b = (uint64_t)(j_in - 1);

        /*
         * Derived decision bits from (i-1) under an iterative inversion rule:
         * Maintain a flip state; for each bit position k (from MSB to LSB over n bits),
         * the derived bit is (bit_of_a XOR flip). Then update flip by XOR with bit_of_a.
         *
         * Output 'L' if derived bit equals corresponding bit of (j-1), else 'R'.
         */
        uint64_t flip = 0;
        for (int k = 0; k < n; k++) {
            int shift = (n - 1) - k;
            uint64_t abit = (a >> shift) & 1ULL;
            uint64_t bbit = (b >> shift) & 1ULL;

            uint64_t derived = abit ^ flip;
            out[k] = (derived == bbit) ? 'L' : 'R';

            flip ^= abit;
        }
        out[n] = '\0';

        puts(out);
    }

    return 0;
}
