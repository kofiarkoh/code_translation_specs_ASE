#include <stdio.h>
#define MOD 1000000007

int main()
{   long long n,k,i,j;
    scanf("%lld%lld",&n,&k);
    long long a[2001];
    for(int i=0;i<n;i++){
        scanf("%lld",&a[i]);
    }
    long long front=0,rear=0;
    for(i=0;i<n;i++){
        for(j=i+1;j<n;j++){
            if(a[i]<a[j]){
                rear++;
            }
        }
        for(j=0;j<i;j++){
            if(a[i]<a[j]){
                front++;
            }
        }
    }
    long long ans=(rear+front)*((k-1)*k/2%MOD)+front*k;
        
    printf("%lld",ans%MOD);

    return 0;
}
