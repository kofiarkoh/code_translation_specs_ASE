#include <stdio.h>

int n;
char data[99][50];
int dp[99][50];

int max(int x, int y)
{
	if (x > y){
		return (x);
	}
	
	return (y);
}

int judge(int i, int j, int sum)
{
	sum += data[i][j];
	
	if (i == n || sum < dp[i][j]){
		return (sum);
	}
	else {
		dp[i][j] = sum;
		
		if (i < n / 2){
			return (max(judge(i + 1, j, dp[i][j]), judge(i + 1, j + 1, dp[i][j])));
		}
		else {
			return (max(judge(i + 1, j - 1, dp[i][j]), judge(i + 1, j, dp[i][j])));
		}
	}
}

int main(void)
{
	int i, j;
	int ans;
	char g;
	
	i = n = g = 0;
	j = 1;
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
	
	n = i;
	
	ans = judge(0, 1, 0);
	
	printf("%d\n", ans);
	
	return (0);
}