#include <stdio.h>

long long int n;

long long int count(long long int a0){
    long long int a,i,S[2]={},C[2]={};
    S[0]=a0;
    S[1]=a0;
    for(i=1;i<n;i++){
        scanf("%lld",&a);
        S[0]+=a;
        S[1]+=a;
        if(i%2==1){
            if(S[0]<=0){
                C[0] += -1*S[0] + 1;
                S[0] = 1;
            }
            if(S[1]>=0){
                C[1] += S[1] + 1;
                S[1] = -1;
            }
        }
        else{
            if(S[0]>=0){
                C[0] += S[0] + 1;
                S[0] = -1;
            }
            if(S[1]<=0){
                C[1] += -1*S[1] + 1;
                S[1] = 1;
            }
        }
    }
    
    if(C[0]<C[1]) return C[0];
    else return C[1];
}

int main()
{
    long long int a0;
    scanf("%lld %lld",&n,&a0);
    printf("%lld\n",count(a0));
    return 0;
}