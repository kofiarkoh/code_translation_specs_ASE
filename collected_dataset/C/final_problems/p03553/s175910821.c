#include <stdio.h>

long long int max(long long int a,long long int b){return a>b?a:b;}

long long int solve(long long int* x,int N,int end){
    long long tmp[N];
    for(int i=0;i<N;i++)tmp[i]=x[i];
    for(int i=end;i>0;i--){
        long long int sum=0;
        for(int j=i;j<=N;j+=i)sum+=tmp[j-1];
        if(sum<0)for(int j=i;j<=N;j+=i)tmp[j-1]=0;
        if(sum==0)for(int j=i;j<=N;j+=i)if(tmp[j-1]!=0){long long int a=solve(tmp,N,i-1);for(int j=i;j<=N;j+=i)tmp[j-1]=0;return max(a,solve(tmp,N,i-1));}
    }
    long long int ans=0;
    for(int i=0;i<N;i++)ans+=tmp[i];
    return ans;
}

int main(void){
    int N;
    scanf("%d",&N);
    long long int a[N];
    for(int i=0;i<N;i++)scanf("%lld",&a[i]);
    printf("%lld",solve(a,N,N));
    return 0;
}
