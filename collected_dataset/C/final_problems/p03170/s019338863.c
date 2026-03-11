#include <stdio.h>

int main(void) {
	int N,K;
	scanf("%d%d",&N,&K);
	int a[100010];
	int dp[100010] = {};
	for(int i=0;i<N;i++){
		scanf("%d",&a[i]);
	}
	
	for(int i=0;i<=K;i++){
		for(int j=0;j<N;j++){
			if(i-a[j]>=0&&dp[i-a[j]]==2){
				dp[i] = 1;
				break;
			}
			dp[i] = 2;
		}
	}
	if(dp[K]==1) printf("First");
	else printf("Second");
	
	return 0;
}