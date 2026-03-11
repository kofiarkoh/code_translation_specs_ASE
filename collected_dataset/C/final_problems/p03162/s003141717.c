#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#define NOT_ZERO 1
#define MAX(a,b) (a>b?a:b)
#define MAX3(a,b,c) MAX(a,MAX(b,c))
#define MIN(a,b) (a<b?a:b)
#define MIN3(a,b,c) MIN(a, MIN(b,c))
typedef long long ll;

int main()
{
    int n, i;
    
    scanf("%d",&n);
    int *a = (int*)malloc(sizeof(int)*n);
    int *b = (int*)malloc(sizeof(int)*n);
    int *c = (int*)malloc(sizeof(int)*n);
    int *A = (int*)malloc(sizeof(int)*n);
    int *B = (int*)malloc(sizeof(int)*n);
    int *C = (int*)malloc(sizeof(int)*n);
    

    for(i=0; i<n; i++) {
        scanf("%d %d %d",a+i, b+i, c+i);
    }

    A[0] = a[0];
    B[0] = b[0];
    C[0] = c[0];
    for(i=1; i<n; i++) {
        A[i] = MAX(B[i-1]+a[i], C[i-1]+a[i]);
        B[i] = MAX(A[i-1]+b[i], C[i-1]+b[i]);
        C[i] = MAX(A[i-1]+c[i], B[i-1]+c[i]);
    }

    int ans = MAX3(A[n-1], B[n-1], C[n-1]);

    printf("%d",ans);
    return 0;
}