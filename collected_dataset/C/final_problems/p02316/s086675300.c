#include <stdio.h>

int main(){
    int N,M;
    scanf("%d %d",&N,&M);
    int value[N];
    int weight[N];
    for(int i=0;i<N;i++){
        scanf("%d %d",&value[i],&weight[i]);
    }
    int dp[M+1];
    dp[0]=0;
    int max=0;
    for(int i=1;i<=M;i++){
        for(int j=0;j<N;j++){
            if(i>=weight[j]){
                if(value[j]+dp[i-weight[j]]>max){
                    max=value[j]+dp[i-weight[j]];
                }
            }
        }
        dp[i]=max;
    }

    printf("%d\n",dp[M]);

    return 0;
}
