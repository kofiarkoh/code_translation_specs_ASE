#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <limits.h>
long long minim(long long a, long long b) { return a < b ? a : b; }
int main(){
    long long n,i,ans=0,leafmax[100010]={},leafmin[100010]={},nodemax[100010]={},a[100010]={};
    scanf("%lld",&n);
    scanf("%lld",&a[0]);
    if(a[0]>=1 && n>=1){
        printf("-1");
        return 0;
    }
    if(a[0]>=1 && n==0){
        printf("-1");
        return 0;
    }
    leafmax[0]=1;
    for(i=1; i<=n; i++){
        scanf("%lld",&a[i]);
        leafmax[i]=2*(leafmax[i-1]-a[i-1]);
        if(leafmax[i]<a[i]){
            printf("-1");
            return 0;
        }
    }
    leafmax[n]=a[n];
    for(i=n-1; i>=0; i--){
        if(leafmax[i]-a[i]>leafmax[i+1])leafmax[i]=leafmax[i+1]+a[i];
    }
    for(i=0; i<=n; i++){
        ans+=leafmax[i];
    }
    printf("%lld",ans);
    return 0;
}