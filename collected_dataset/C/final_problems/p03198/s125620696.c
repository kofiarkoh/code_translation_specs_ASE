

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

llong int plami(llong int x, llong int y) {
	if (y == 1) {
		if (x >= 0) {
			return 1;
		}
		else {
			return 0;
		}
	}
	else if (y == -1) {
		if (x <= 0) {
			return 1;
		}
		else {
			return 0;
		}
	}
}

llong int nibai(llong int x, llong int y) {
	if (y == 1) {
		if (x >= 0) {
			return -2 * x;
		}
		else {
			return x;
		}
	}
	else if (y == -1) {
		if (x <= 0) {
			return -2 * x;
		}
		else {
			return x;
		}
	}
}


int main(void)
{
	//変数の宣言
	int n;
	static llong int a[100010];
	static llong int b[100010];
	static llong int c[100010];
	static llong int bfugo[100010];
	static llong int cfugo[100010];
	static llong int minus[100010][40];
	static llong int plus[100010][40];
	
	int templ;
	int tempr;

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

	//データの読み込み
	scanf("%d", &n);
//	scanf_s("%d", &n);

	for (i = 0;i < n;i++) {
		scanf("%d", &a[i]);
//		scanf_s("%d", &a[i]);
	}

	//	printf("nは%dです\n", n);
	//	printf("データの読み込み終了\n");
		//実際の処理

		//n=1の場合分けいる？



	flag = 0;
	temp = -1;
	for (i = 0;i < n;i++) {
		if (a[i] == 0) {
			if (temp == -1) {
				templ = i;
				tempr = i;
				flag = 1;
			}
			else if (i - tempr == 1) {
				tempr = i;
			}
			else {
				flag = 2;
				break;
			}
		}
	}
	
	if (flag == 2) {
		printf("-1");
		return 0;
	}
	else if (flag == 0) {
		templ = n;
		tempr = 0;
	}
	
	for(i=0;i<n;i++){
		if(a[i]>=0){
			b[i]=-2*a[i];
			bfugo[i]=1;
		}else{
			b[i]=a[i];
			bfugo[i]=0;
		}
	}
	
	
	for(i=0;i<n;i++){
		if(a[i]<=0){
			c[i]=-2*a[i];
			cfugo[i]=1;
		}else{
			c[i]=a[i];
			cfugo[i]=0;
		}
	}
	
	//minus
	for (i = 0;i < templ;i++) {
		if (i == 0) {
			for(j=0;j<=30;j++){
				minus[0][j]=bfugo[0]+2*j;
			}
		}else{
			temp = 0;
			temp1 =b[i-1];
			temp2 =b[i];
			if(temp1>temp2){
				while (temp1 > temp2) {
					temp++;
					temp1 = temp1 * 4;
				}
			}else if(temp1<temp2){
				while(temp1<temp2){
					temp--;
					temp2=temp2*4;
				}
			}
			
			for(j=0;j<=15;j++){
				minus[i][j]=minus[i-1][max(0,j+temp)]+bfugo[i]+2*j;
			}
			for(j=16;j<=30;j++){
				minus[i][j]=minus[i][j-1]+2*(i+1);
			}
		}
	}
	
	
	
	//plus
	for (i = n-1;i >= tempr;i--) {
		if (i == n-1) {
			for(j=0;j<=30;j++){
				plus[i][j]=cfugo[i]+2*j;
			}
		}else{
			temp = 0;
			temp1 =c[i+1];
			temp2 =c[i];
			if(temp1<=temp2){
				while (temp1 < temp2) {
					temp++;
					temp1 = temp1 * 4;
				}
			}else if(temp1>temp2){
				while(temp1>temp2){
					temp--;
					temp2=temp2*4;
				}
			}
			
			for(j=0;j<=15;j++){
				plus[i][j]=plus[i+1][max(0,j+temp)]+cfugo[i]+2*j;
			}
			for(j=16;j<=30;j++){
				plus[i][j]=plus[i][j-1]+2*(n-i);
			}
		}
	}
	
//	for(i=0;i<n;i++){
//		for(j=0;j<5;j++){
//			printf("minus[%d][%d]=%lld,plus[%d][%d]=%lld\n",i,j,minus[i][j],i,j,plus[i][j]);
//		}
//		printf("\n");
//	}
	
	
	if (flag == 1) {
		ans = 0;
		if (templ != 0) {
			ans = ans + minus[templ - 1][0];
		}
		if (tempr != n - 1) {
			ans = ans + plus[tempr + 1][0];
		}
		printf("%lld", ans);
	}
	else {
		for (i = 0;i < n;i++) {
			if (i == 0) {
				ans = min(minus[n - 1][0], plus[0][0]);
			}
			else {
				if (minus[i - 1][0] + plus[i][0] < ans) {
					ans = minus[i - 1][0] + plus[i][0];
				}
			}
		}
		printf("%lld", ans);
	}

	//	printf("計算部分終了\n");
		//出力

		//printf("%lld",ans);


	//	printf("結果の出力終了\n");

	return 0;

}

