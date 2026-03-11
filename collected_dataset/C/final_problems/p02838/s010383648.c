#include <stdio.h>

int number_of_bits[60];

int main (void) {
    int n, i, j;
    long long unsigned int res = 0, tmp;

    scanf("%d", &n);

    for (i = 0; i < n; i++) {
        scanf("%llu", &tmp);
        for (j = 0; tmp; j++, tmp >>= 1) {
            if (tmp % 2) number_of_bits[j]++;
        }
    }

    for (i = 59; i >= 0; i--) {
        res = res * 2 % 1000000007 + number_of_bits[i] * (n - number_of_bits[i]) % 1000000007;
    }

    printf("%llu\n", res);
}
