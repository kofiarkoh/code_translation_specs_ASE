#include <stdio.h>

long long calcul(long long k, long long r, long long b) {
    long long total_used = 0;
    long long p, q;

    // Iterate over possible values of p and q to calculate total resources used
    for (p = 0; p <= k; p++) {
        // Calculate q based on the remaining allocation
        q = k - p;

        // Calculate resources used based on p and q
        if (q > 0) {
            total_used += (p * (p + 1)) / 2; // Sum of first p integers
            total_used += (q * (q + 1)) / 2; // Sum of first q integers
        } else {
            total_used += (p * (p + 1)) / 2; // Only p contributes
        }

        // Check if total resources used exceed available resources
        if (total_used > r + b) {
            return total_used; // Early exit if resources exceeded
        }
    }
    return total_used;
}

int ispossible(long long k, long long r, long long b) {
    long long total_used = calcul(k, r, b);
    return total_used <= r + b; // Check if total used is within available resources
}

long long max_allocation(long long r, long long b) {
    long long low = 0, high = r + b; // Set search bounds
    long long result = -1; // Initialize result

    // Binary search for maximum feasible k
    while (low <= high) {
        long long mid = low + (high - low) / 2; // Midpoint for binary search

        if (ispossible(mid, r, b)) {
            result = mid; // Update result if feasible
            low = mid + 1; // Search for a higher k
        } else {
            high = mid - 1; // Search for a lower k
        }
    }

    return result - 1; // Return k - 1 as per specification
}

int main() {
    long long r, b;
    // Assume valid input is provided
    scanf("%lld %lld", &r, &b); // Read resources

    // Output the maximum feasible allocation minus one
    printf("%lld\n", max_allocation(r, b));
    return 0;
}
