#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<stdlib.h>
#include<string.h>

void LCS(char *x, char *y){
	int m = strlen(x+1);
	int n = strlen(y+1);
	int i, j, **c;

	c = malloc(sizeof(int)*(m+900));
	for (i = 0; i <= m; i++)
		c[i] = malloc(sizeof(int)*(n+1));
	
	for (i = 0; i <= m; i++){
		for (j = 0; j <= n; j++){
			c[i][j] = 0;
		}
	}
	
	for (i = 1; i <= m; i++){
		for (j = 1; j <= n; j++){
			if (x[i] == y[j])
				c[i][j] = c[i - 1][j - 1] + 1;
			else if (c[i - 1][j] >= c[i][j - 1])
				c[i][j] = c[i - 1][j];
			else
				c[i][j] = c[i][j - 1];
		}
	}
	
	printf("%d\n", c[i-1][j-1]);
	for (i = 0; i < m; i++)free(c[i]);
	free(c);
	
}

int main(){
	int i, n;
	char x[1001], y[1001];

	scanf("%d", &n);
	for (i = 0; i < n; i++){
		scanf("%s%s", x + 1, y + 1);
		LCS(x, y);
	}
	return 0;
}