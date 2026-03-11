#include <stdio.h>

#define SIZE 250
#define NUM 801

const int Mod = 998244353;

int main()
{
	int i, N, Q;
	char S[200001];
	scanf("%d %d", &N, &Q);
	
	char block[NUM];
	int j, k, l, r, d, q, n = (N - 1) / SIZE, m = N - n * SIZE, flag[NUM];
	long long ans, ans_block[NUM] = {}, fill_block[10], pow[200001];
	for (i = 1, pow[0] = 1; i < N; i++) pow[i] = pow[i-1] * 10 % Mod;
	for (i = 1; i <= 9; i++) {
		for (j = 1, fill_block[i] = i; j < SIZE; j++) fill_block[i] = (fill_block[i] * 10 + i) % Mod;
	}
	for (i = 0; i <= n; i++) {
		block[i] = 1;
		flag[i] = 1;
		if (i < n) ans_block[i] = fill_block[1] * pow[m+(n-i-1)*SIZE] % Mod;
		if (i == n) for (j = 0, ans_block[i] = 0; i * SIZE + j < N; j++) ans_block[i] = (ans_block[i] * 10 + 1) % Mod;
	}
	
	for (q = 1; q <= Q; q++) {
		scanf("%d %d %d", &l, &r, &d);
		l--;
		r--;
		i = l / SIZE;
		if (flag[i] == 1) {
			for (j = i * SIZE; j < N && j < (i + 1) * SIZE; j++) S[j] = block[i];
			flag[i] = 0;
		}
		for (j = l, i = j / SIZE; j <= r && j < (i + 1) * SIZE; j++) {
			ans_block[i] = (ans_block[i] + pow[N-j-1] * (Mod - S[j]) % Mod + pow[N-j-1] * d) % Mod;
			S[j] = d;
		}
		if (j <= r) {
			for (i++; i < r / SIZE; i++) {
				flag[i] = 1;
				block[i] = d;
				ans_block[i] = fill_block[d] * pow[m+(n-i-1)*SIZE] % Mod;
			}
			if (flag[i] == 1) {
				for (j = i * SIZE; j < N && j < (i + 1) * SIZE; j++) S[j] = block[i];
				flag[i] = 0;
			}
			for (j = i * SIZE; j <= r; j++) {
				ans_block[i] = (ans_block[i] + pow[N-j-1] * (Mod - S[j]) % Mod + pow[N-j-1] * d) % Mod;
				S[j] = d;
			}
		}
		
		for (i = 0, ans = 0; i < n; i++) ans += ans_block[i];
		ans = (ans + ans_block[n]) % Mod;
		printf("%lld\n", ans);
	}
	fflush(stdout);
	return 0;
}