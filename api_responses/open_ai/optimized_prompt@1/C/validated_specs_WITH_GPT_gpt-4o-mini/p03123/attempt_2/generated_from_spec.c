#include <stdio.h>

long long ispossible(long long r, long long b, long long k) {
    // Check if there exist non-negative integers p and q such that:
    // p + q = k
    // r >= p * (p + 1) / 2
    // b >= q * (q + 1) / 2
    // This translates to checking if the resources can satisfy the allocation.
    for (long long p = 0; p <= k; p++) {
        long long q = k - p;
        if (p * (p + 1) / 2 <= r && q * (q + 1) / 2 <= b) {
            return 1; // Feasible allocation found
        }
    }
    return 0; // No feasible allocation
}

long long max_allocation(long long r, long long b) {
    long long low = 0, high = r + b; // Maximum possible k is r + b
    long long result = -1; // Initialize result to -1 (k - 1 will be returned)

    while (low <= high) {
        long long mid = low + (high - low) / 2; // Midpoint for binary search
        if (ispossible(r, b, mid)) {
            result = mid; // Update result if mid is feasible
            low = mid + 1; // Search in the upper half
        } else {
            high = mid - 1; // Search in the lower half
        }
    }

    return result - 1; // Return k - 1 as per specification
}

int main() {
    long long r, b;
    // Assuming valid input is provided as per the specification
    scanf("%lld %lld", &r, &b);
    long long result = max_allocation(r, b);
    printf("%lld\n", result); // Output the result
    return 0;
}
