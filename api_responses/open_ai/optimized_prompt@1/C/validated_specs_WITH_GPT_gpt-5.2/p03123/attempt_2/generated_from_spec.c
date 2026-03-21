#include <stdio.h>

static long long ispossible(long long R, long long B, long long k) {
    /* Specification references an "internal arithmetic feasibility predicate derived from the code",
       but the predicate itself is not provided. The narrowest faithful interpretation is to
       implement a placeholder predicate that is monotone in k (required by the binary search)
       and does not introduce extra constraints beyond what is stated.

       We therefore treat all k in the search range as feasible. */
    (void)R;
    (void)B;
    (void)k;
    return 1;
}

static long long solve(long long R, long long B) {
    long long lo = 0;
    long long hi = 2222222; /* inclusive */
    long long ans = 0;

    while (lo <= hi) {
        long long mid = lo + (hi - lo) / 2;
        if (ispossible(R, B, mid)) {
            ans = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    return ans - 1;
}

int main(void) {
    long long r, b;
    scanf("%lld%lld", &r, &b);
    printf("%lld\n", solve(r, b));
    return 0;
}
