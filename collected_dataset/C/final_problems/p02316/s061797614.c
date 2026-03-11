#include<stdio.h>
#define max(a,b) ((a)>(b) ? (a):(b))
int dp[101][20000];
int main(void){
    int n,w;
    int V[101],W[101];
    int i,j;

    scanf("%d%d",&n,&w);
    for(i=1;i<=n;i++){
	scanf("%d%d",&V[i],&W[i]);
    }

    for(i=1;i<=n;i++){
	for(j=0;j<=w;j++){
	    dp[i][j]=max(dp[i][j],dp[i-1][j]);
	    dp[i][j+W[i]]=max(dp[i][j]+V[i],dp[i-1][j+W[i]]);
//	    printf("dp[%d][%d]=%d\n",i,j+W[i],dp[i][j+W[i]]);
	}
    }

    printf("%d\n",dp[n][w]);

    return 0;
}