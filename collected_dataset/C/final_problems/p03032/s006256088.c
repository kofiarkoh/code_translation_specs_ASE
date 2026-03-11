#include <stdio.h>
#include <string.h>

int V[50];
long long int dp[50][50][100];

long long int MAX(long long int a, long long int b, long long int c) {
	if (a >= b && a >= c) {
		return a;
	}
	else if (b >= a && b >= c) {
		return b;
	}
	else {
		return c;
	}
}

long long int suteru(int sum, int k, int fu[50], int cnt) {
	int i;
	for (i = cnt - 1; i >= 0 && k != 0; i--) {
		sum = sum - fu[i];
		k--;
	}
	return sum;
}

long long int rec(long long int sum, int left, int right, int value, int k, int* fu, int cnt) {
	
	int i;
	int temp[50];
	
	if (dp[left][right][k] != -1) {
		return dp[left][right][k];
	}

	for (i = cnt - 1; i >= 0; i--) {
		temp[i] = fu[i];
	}

	if (value < 0) {
		for (i = cnt; i > 0 && temp[i - 1] < value; i--) {
			temp[i] = temp[i - 1];
		}		
		temp[i] = value;
		cnt++;
	}

	sum = sum + value;

	if (k == 0 || left > right) {
		return suteru(sum, k, temp, cnt);
	}

	return dp[left][right][k] = MAX(suteru(sum, k, temp, cnt), rec(sum, left + 1, right, V[left], k - 1, temp, cnt), rec(sum, left, right - 1, V[right], k - 1, temp, cnt));
}

int main() {
	int N, K, i, fu[50], cnt = 0;
	long long int vmax = 0;

	memset(fu, 0, sizeof(fu));
	memset(dp, -1, sizeof(dp));

	scanf("%d %d", &N, &K);
	for (i = 0; i < N; i++) {
		scanf("%d", &V[i]);
	}

	vmax = MAX(0, rec(0, 1, N - 1, V[0], K - 1, fu, cnt), rec(0, 0, N - 2, V[N - 1], K - 1, fu, cnt));

	printf("%lld\n", vmax);

	return 0;
}