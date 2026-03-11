#include <stdio.h>
#define MOD 1000000007;
typedef struct{
    long long front;
    long long rear;
}set;


int main()
{   long long n,k;
    scanf("%lld%lld",&n,&k);
    long long a[2001];
    for(int i=0;i<n;i++){
        scanf("%lld",&a[i]);
    }
    set b[2001];
    for(int i=0;i<2000;i++){
        b[i].front=0;
        b[i].rear=0;
    }
    for(int i=0;i<n;i++){
        for(int j=0;j<i;j++){
            if(a[i]<a[j]){
                b[i].front++;
                //printf("b[%d].f increased @%d\n",i,j);
            }
        }for(int j=i+1;j<n;j++){
            if(a[i]<a[j]){
                b[i].rear++;
                //printf("b[%d].r increased @%d\n",i,j);
            }
        }
    }
    long long ans=0;
    for(int i=0;i<n;i++){
        ans+=(b[i].front*(k+1)*k/2)%MOD;
        ans%=MOD;
        ans+=(b[i].rear*(k-1)*k/2)%MOD;
        ans%=MOD;
    }
    

    
    printf("%lld",ans);

    return 0;
}
