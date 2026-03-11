#include <stdio.h>

int main() {

    int i, n;
    double x, slope, max_slope = 0.0, xi, hi;
    scanf("%d %lf", &n, &x);

    for ( i=0; i<n; ++i ) {
	scanf("%lf %lf", &xi, &hi);
	slope = hi/xi;
	if ( slope > max_slope ) max_slope = slope;
    }

    printf("%lf\n", x*max_slope);
    return 0;
}

