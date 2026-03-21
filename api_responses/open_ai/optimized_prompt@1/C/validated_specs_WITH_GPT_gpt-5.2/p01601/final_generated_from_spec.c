#include <stdio.h>
#include <limits.h>

/* Reverse decimal digits of x, ignoring any sign.
   Produces a non-negative integer derived from the decimal digits. */
static int reverse_digits_ignore_sign(int x) {
    /* Use long long to avoid intermediate overflow when negating INT_MIN. */
    long long t = (long long)x;
    if (t < 0) t = -t;

    long long rev = 0;
    while (t > 0) {
        rev = rev * 10 + (t % 10);
        t /= 10;
    }
    return (int)rev;
}

static int is_decimal_palindrome_nonneg(int x) {
    if (x < 0) return 0;
    return x == reverse_digits_ignore_sign(x);
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Unspecified behavior per spec if input cannot be read; exit quietly. */
        return 0;
    }

    /* Search outward from n by increasing distance d.
       At each distance, check smaller candidate first to satisfy tie-break. */
    for (int d = 0;; d++) {
        /* Candidate on the lower side: n - d, but only if non-negative. */
        if (n >= 0) {
            int low = n - d; /* For non-negative n, this stays within int for d <= n. */
            if (low >= 0 && is_decimal_palindrome_nonneg(low)) {
                printf("%d\n", low);
                return 0;
            }
        } else {
            /* For negative n, low = n - d is always negative; skip (non-negative required). */
        }

        /* Candidate on the upper side: n + d, but must be non-negative. */
        {
            int high = n + d; /* If this overflows, behavior is undefined per spec. */
            if (high >= 0 && is_decimal_palindrome_nonneg(high)) {
                printf("%d\n", high);
                return 0;
            }
        }
    }
}
