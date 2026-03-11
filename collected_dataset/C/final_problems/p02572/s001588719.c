#include<stdio.h>
#define max(a,b) ((a)>(b)?(a):(b))
typedef long long int ll;
#define MOD 1000000007
int main() {
    int n;
    scanf("%d",&n);
    long a[200000];
    long aa[200000]={0};
    long ans=0;
  	for (int i=0;i<n;i++) {
        scanf("%ld",&a[i]);
        if(i==0) aa[i]=a[i]%MOD;
        else aa[i]=(aa[i-1]+a[i])%MOD;
    }
    for (int i=0;i<n-1;i++) {
        ans=(ans+(a[i]*(aa[n-1]-aa[i]))%MOD)%MOD;
    }
    printf("%ld",ans);
    return 0;
}