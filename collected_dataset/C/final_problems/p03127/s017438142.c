#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

int asort(const void *A,const void *B){
    return *(long long*)A>*(long long*)B?1:0;
}

int dsort(const void *A,const void *B){
    return *(long long*)A<*(long long*)B?1:0;
}

long long gcd(long long A,long long B){
    return B?gcd(B,A%B):A;
}

int main(void){
    int n;
    scanf("%d",&n);
    long long a[n];
    for (int i=0; i<n; i++){
        scanf("%lld",&a[i]);
    }
    qsort (a,n,sizeof(long long),dsort);
    long long ans=gcd(a[0],a[1]);
    for (int i=0; i<n; i++){
        ans=gcd(ans,a[i]);
    }
    printf("%lld\n",ans);
    return 0;
}
