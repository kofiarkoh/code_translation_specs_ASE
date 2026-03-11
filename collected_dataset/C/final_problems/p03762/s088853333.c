#include <stdio.h>
#include <stdlib.h>
#define D 1000000007
int main()
{
    long m,n,i;
    long long *x,*y;
    unsigned long long w=0,h=0;
    scanf("%ld %ld",&n,&m);
    x=(long long *)malloc(sizeof(x)*n);
    y=(long long *)malloc(sizeof(y)*m);
    for(i=0;i<n;i++){
        scanf("%lld",&x[i]);
        w+=i*x[i]-(n-i-1)*x[i];
    }
    w%=D;
    for(i=0;i<m;i++){
        scanf("%lld",&y[i]);
        h+=i*y[i]-(m-i-1)*y[i];
    }
    h%=D;
    printf("%lld\n",w*h%D);
    free(x);
    free(y);
    return 0;
}