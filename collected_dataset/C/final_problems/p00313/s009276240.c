#include <stdio.h>

#define MAX_N 100

int main() {

    int i, x, y, z, e, n, ans = 0;
    int A[MAX_N], B[MAX_N], C[MAX_N];

    scanf("%d", &n);
    for ( i=0; i<n; ++i ) A[i] = B[i] = C[i] = 0;

    scanf("%d", &x);
    for ( i=0; i<x; ++i ) {
	scanf("%d", &e);
	A[e-1] = 1;
    }

    scanf("%d", &y);
    for ( i=0; i<y; ++i ) {
	scanf("%d", &e);
	B[e-1] = 1;
    }

    scanf("%d", &z);
    for ( i=0; i<z; ++i ) {
	scanf("%d", &e);
	C[e-1] = 1;
    }

    for ( i=0; i<n; ++i ) {
	if ( (!A[i] && C[i]) || (B[i] && C[i]) ) ++ans;
    }
    printf("%d\n", ans);

    return 0;
}

