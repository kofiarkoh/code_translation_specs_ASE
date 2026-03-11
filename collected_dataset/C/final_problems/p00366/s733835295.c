#include <stdio.h>

int timing[100000];
int divisor[10000];

int main() {

    int max_t = 0, ndiv = 0, adj = 0;
    int i, j, n;
    scanf("%d", &n);

    for ( i=0; i<n; ++i ) {
	scanf("%d", &timing[i]);
	if ( timing[i] > max_t ) max_t = timing[i];
    }

    for ( i=1; i<=max_t>>1; ++i ) {
	if ( max_t%i == 0 ) divisor[ndiv++] = i;
    }
    divisor[ndiv++] = max_t;

    for ( i=0; i<n; ++i ) {
	for ( j=0; j<ndiv; ++j ) {
	    if ( divisor[j] >= timing[i] ) {
		adj += divisor[j] - timing[i];
		break;
	    }
	}
    }

    printf("%d\n", adj);
    return 0;
}

