#include<stdio.h>
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
#define S(type,x) fscanf(stdin,"%"#type,&x)
//#define MAX(a,b) a>b?a:b
//#define MIN(a,b) a<b?a:b
unsigned long long dp[11][1001];	
int main() {
	dp[0][0] = 1;
	int i, j, k;
	for (i = 0; i < 101; i++)
		for (j = 9; j >= 0; j--)
			for (k = 0; k + i < 1001; k++)
				dp[j + 1][k + i] += dp[j][k];
	int n, s;
	while (fscanf(stdin, "%d%d", &n, &s), n, s)
		P(d, dp[n][s]);
	return 0;
}