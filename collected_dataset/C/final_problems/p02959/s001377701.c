#include <stdio.h>
#include <stdlib.h>

int main(){
    long long n,i,count = 0;
    long long  a[100010],b[100010];

    scanf("%lld",&n);
    for(i = 1; i <= n+1 ; i++){
        scanf("%lld",&a[i]);
    }
    for(i = 1; i <= n ; i++){
        scanf("%lld",&b[i]);
    }
    for(i = 1; i <= n ; i++){
        if(a[i] <= b[i]){
            count += a[i];
            b[i] -= a[i];
            a[i] = 0;
        }else{
            count += b[i];
            a[i] -= b[i];
            b[i] = 0;
        }
        if(a[i+1] <= b[i]){
            count += a[i+1];
            b[i] -= a[i+1];
            a[i] = 0;
        }else{
            count += b[i];
            a[i+1] -= b[i];
            b[i] = 0;
        }

    }




    printf("%lld\n",count);
    return 0;
}