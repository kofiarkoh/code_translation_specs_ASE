#include <stdio.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        return 0; /* unspecified behavior on malformed input; exit quietly */
    }

    /* Fixed storage capacity: at most 100 vertices (precondition). */
    int x[100], y[100];

    for (int i = 0; i < n; i++) {
        if (scanf("%d %d", &x[i], &y[i]) != 2) {
            return 0; /* unspecified behavior on malformed input; exit quietly */
        }
    }

    /* Shoelace formula signed area sum:
       sum_{i=0..n-1} (x[i]*y[i+1] - y[i]*x[i+1]), with wraparound to 0. */
    long long sum = 0;
    for (int i = 0; i < n; i++) {
        int j = (i + 1) % n;
        sum += (long long)x[i] * (long long)y[j] - (long long)y[i] * (long long)x[j];
    }

    /* Output is half of the signed sum, printed with one digit after decimal. */
    double area = (double)sum / 2.0;
    printf("%.1f\n", area);

    return 0;
}
