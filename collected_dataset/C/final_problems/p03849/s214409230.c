//#include "pch.h"
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define llong long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q) ((p)<(q)?(p):(q))
#define max(p,q) ((p)>(q)?(p):(q))
#define INF 1000000000000//10^12

llong ketamax[80];
llong sanjo[80];
llong nijo[80];
int roop=0;
int dp[100][5];

int init(int x, int y) {
	int i, j;
	for (i = 1;i <= x;i++) {
		for (j = 0;j < y;j++) {
			dp[i][j] = 0;
		}
	}
	return 0;
}

llong rec(llong keta, llong sum) {

	int r;
	int i;
	llong ret = 0;

	roop++;
	//printf("keta=%lld,sum=%lld,roop=%d\n",keta,sum,roop);

		//再帰の終了条件と返り値を書く

	if (sum >= ketamax[keta]) {
		return sanjo[keta];
	}else if(sum==0){
		return 1;
	}else if (keta == 1) {
		return sum+1;
	}

	//今の桁の数字を数値に直すのとroop回数を決める

	//x=(int)n[keta]-'0';

	r = min(2,sum/nijo[keta]);

	if (dp[keta][r] > 0) {
		return dp[keta][r];
	}


	//再帰先(rec())を書く
	for (i = 0;i <= r;i++) {
		ret = ret + rec(keta - 1, sum - i*nijo[keta]);
		ret = ret % 1000000007;
	}

	dp[keta][r] = ret;
	return ret;
}

int main(void)
{
	//変数の宣言
	llong n;

	//よく使う変数
	int i, j, k, l;
	int flag = 0;
	llong int ans = 0;
	int count = 0;
	llong int temp, temp1, temp2;
	int max, min;
	int len;
	int sum = 0;
	int under, upper, middle;
	int dummy;

	//データの読み込み
	scanf("%lld", &n);
	//scanf_s("%lld", &n);

	//	printf("nは%dです\n", n);
	//	printf("データの読み込み終了\n");
	//実際の処理

	temp = 1;
	count = 1;
	ketamax[1] = 2;
	nijo[1] = 1;
	sanjo[1] = 3;
	
	while (temp < n) {
		temp = temp * 2;
		count++;
		ketamax[count] = temp*4-2;
		nijo[count] = nijo[count - 1] * 2;
	}

	for (i = 1;i <= count;i++) {
		if (i == 1) {
			sanjo[i] = 3;
		}
		else {
			sanjo[i] = (sanjo[i - 1] * 3) % 1000000007;
		}
	}

	init(80, 4);
	ans = rec(count, n);

	//	printf("計算部分終了\n");
	//出力

	printf("%lld", ans);


	//	printf("結果の出力終了\n");

	return 0;

}

