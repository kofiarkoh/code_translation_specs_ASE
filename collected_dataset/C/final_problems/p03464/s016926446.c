#include <stdio.h>
int main(void){
    long long k,max=2,min=2;
    scanf("%lld",&k);
    long long a[k];
    for (int i=0; i<k; i++){
        scanf("%lld",&a[i]);
    }
    for (int i=k-1; i>=0; i--){
        min=((min+a[i]-1)/a[i])*a[i];
        if (min>(max/a[i])*a[i]){
            printf("-1\n");
            return 0;
        }
        max=((max/a[i])*a[i])+a[i]-1;
    }
    printf("%lld %lld\n",min,max);
    return 0;
}