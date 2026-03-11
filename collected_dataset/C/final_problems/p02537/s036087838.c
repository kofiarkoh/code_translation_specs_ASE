#include<stdio.h>
#include<stdlib.h>

int main(void){
    long int n,k;
    scanf("%ld %ld",&n,&k);
    long int a[n-1];
    for(long int i=1; i<=n; i++) scanf("%ld",&a[i-1]);

    long int kiroku,koma,ans;
    ans = 1;

    for(long int i=1; i<n; i++){
        kiroku = 1;
        koma   = a[i-1];
        for(long int j=i; j<=n; j++){
            if(labs(a[j-1]-a[j]) <= k){
                kiroku += 1;
                koma   =  a[j]; 
            }else{
                koma = a[j+1];
            }
        }
        if(kiroku > ans)    ans = kiroku;
    }

    printf("%ld",ans);
    return 0;
}