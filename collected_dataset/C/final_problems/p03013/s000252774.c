#include <stdio.h>
int main(void){
    int n,m,x,i,broken[100010]={0},dp[100010]={0};
    scanf("%d%d",&n,&m);
    for(i=0;i<m;i++){
        scanf("%d",&x);
        broken[x]=1;
    }
    dp[n]=1;
    dp[n+1]=0;
    for(i=n-1;i>=0;i--){
        if(broken[i]==1) continue;
        dp[i] = (dp[i+1]+dp[i+2])%1000000007;
    }
    printf("%d",dp[0]);
}