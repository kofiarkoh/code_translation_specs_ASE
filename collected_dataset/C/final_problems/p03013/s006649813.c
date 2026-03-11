#include<stdio.h>
#include<stdlib.h>
int main(){
    int n,m;
    scanf("%d %d",&n,&m);
    int a[m];
    for(int i = 0;i < m;i++){
        scanf("%d",&a[i]);
    }
    int dp[n + 1];
    for(int i = 0;i < (n + 1);i++){
        dp[i] = 0;
    }
    dp[0] = 1;
    for(int i = 1;i < (n + 1);i++){
        
        if(i == 1){
            dp[i] = 1;
            }else{
            dp[i] += dp[i - 1];
            dp[i] += dp[i - 2];
            }
        if(dp[i] > 1000000007){
            dp[i] %= 1000000007;
        }    


        for(int j = 0;j < m;j++){
            if(i == a[j]){
                dp[i] = 0;
            }
        }
    }
    printf("%d",dp[n]);
    return 0;
}