#include <stdio.h>

char data[99][51];

int max(int x, int y)
{
	if (x > y){
		return (x);
	}
	
	return (y);
}

int judge(int n)
{
	int i, j;
	int dp[100][51];
	
	for (i = 1; i <= n; i++){
		for (j = 1; j <= i; j++){
			if (i <= n / 2 + 1){
				dp[i][j] = data[i][j] + max(dp[i - 1][j], dp[i - 1][j - 1]);
			}
			else {
				dp[i][j] = data[i][j] + max(dp[i - 1][j], dp[i - 1][j + 1]);
			}
		}
		if (i <= n / 2 + 2){
			dp[i][j - (i - (n / 2 + 1))] = 0;
		}
	}
	
	return (dp[n][1]);
}

int main(void)
{
	int i, j;
	char g;
	
	g = 0;
	i = j = 1;
	while (scanf("%d%c", &data[i][j], &g) != EOF){
		if (g == ','){
			j++;
			g = 0;
		}
		else {
			data[i][j + 1] = 0;
			i++;
			data[i][0] = 0;
			j = 1;
		}
	}
	
	printf("%d\n", judge(i));
	
	return (0);
}