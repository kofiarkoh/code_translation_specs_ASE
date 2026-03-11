// AOJ 2025: Eight Princes
// 2017.11.8 bal4u@uu

#include <stdio.h>

long long dp1[10][101][2][2];
long long dp2[10][101][3][3];

int main()
{
	int n, i, j, k;
	long long ans;

	dp1[0][1][0][0] = dp1[1][1][1][1] = 1;
	for (i = 0; i < 9; i++) for (j = 1; j < 100; j++) {
		for (k = 0; k < 2; k++) {
			dp1[i][j+1][k][0] += dp1[i][j][k][0] + dp1[i][j][k][1];
			dp1[i+1][j+1][k][1] += dp1[i][j][k][0];
		}
	}
	dp2[0][1][0][0] = dp2[1][1][1][1] = dp2[1][1][2][2] = 1;
	for (i = 0; i < 9; i++) for (j = 1; j < 100; j++) {
		for (k = 0; k < 3; k++) {
			dp2[i][j+1][k][0] += dp2[i][j][k][0] + dp2[i][j][k][1] + dp2[i][j][k][2];
			dp2[i+1][j+1][k][1] += dp2[i][j][k][0] + dp2[i][j][k][2];
			dp2[i+1][j+1][k][2] += dp2[i][j][k][0] + dp2[i][j][k][1];
		}
	}

	while (scanf("%d", &n) && n > 0) {
		if (n & 1) ans = dp1[8][n][0][0] + dp1[8][n][0][1] + dp1[8][n][1][0];
		else {
			for (ans = 0, i = 0; i < 3; i++) for (j = 0; j < 3; j++)
				if (i*j != 2) ans += dp2[8][n/2][i][j];
		}
		printf("%lld\n", ans * 40320);
	}
	return 0;
}