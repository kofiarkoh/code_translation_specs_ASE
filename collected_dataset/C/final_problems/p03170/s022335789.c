#include<stdio.h>
#define N 100
#define K 100000

int a[N+1];		//石をとれる個数の集合
int dp[K+1];	//=1勝ち状態,=0負け状態

int main(void)
{
	int i;
	int j;	//for文を回す用のi,j
	int k;	//山石の個数
	int n;  //Aの要素数

	/*入力*/
	scanf("%d%d", &n, &k);
	for(i=1; i<=n; i++){
		scanf("%d", &a[i]);
	}

	/*処理*/
	dp[0] = 0;
	for(i=1; i<=k; i++){
		for(j=1; j<=n; j++){
			if((dp[i-a[j]] == 0) && ((i-a[j]) >= 0)){
				dp[i] = 1;
			}
		}
	}
	
	/*出力*/
	if(dp[k] == 1){
		printf("First\n");
	}else{
		printf("Second\n");
	}

	return 0;
}