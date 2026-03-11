#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p 998244353

int *fact, *fact_inv, *pow2;

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

int power(int a, int N){
	if(N == 0){
		return 1;
	}
	else if(N % 2 == 0){
		return power(MOD(a * a), N / 2);
	}
	else{
		return MOD(a * power(a, N - 1));
	}
}

int invers(int a){
	return power(a, p - 2);
}

int combination(int N, int k){
	if(k < 0 || N < k){
		return 0;
	}
	else{
		return MOD(fact[N] * MOD(fact_inv[k] * fact_inv[N - k]));
	}
}

int solve1(int K, int N, int s){
	int j, ans = 0;
	for(j = 0; j <= K - s / 2; j++){
		ans = MOD(ans + MOD(combination(K - s / 2, j) * MOD(pow2[j] * combination(N + s - K - 2, N - j))));
	}
	return ans;
}

int solve2(int K, int N, int s){
//	printf("(K, N, s) = (%lld, %lld, %lld)\n", K, N, s);
	int j, ans = 0;
	for(j = 0; j <= s / 2; j++){
//		printf("combination(%lld, %lld) = %lld\n", N + K - s, N - j, combination(N + K - s, N - j));
		ans = MOD(ans + MOD(combination(s / 2, j) * MOD(pow2[j] * combination(N + K - s, N - j))));
//		printf("ans[%lld] = %lld\n", j, ans);
	}
	return ans;
}

signed main(){
	int K, N, i, ans = 0;
	scanf("%lld%lld", &K, &N);
	fact = (int *)malloc(sizeof(int) * (N + K + 1));
	fact_inv = (int *)malloc(sizeof(int) * (N + K + 1));
	pow2 = (int *)malloc(sizeof(int) * (N + K + 1));
	fact[0] = 1;
	fact_inv[0] = 1;
	pow2[0] = 1;
	for(i = 1; i <= N + K; i++){
		fact[i] = MOD(i * fact[i - 1]);
		fact_inv[i] = invers(fact[i]);
		pow2[i] = MOD(2 * pow2[i - 1]);
	}
	for(i = 2; i <= 2 * K; i++){
//		printf("i = %lld\n", i);
		if(i % 2 == 0){
			if(K < i / 2){
				ans = combination(N + K - 1, N);
			}
			else if(K <= i - 1){
//				printf("test2\n");
				ans = MOD(solve1(K - 1, N, i - 1) + solve1(K - 1, N - 1, i - 1));
			}
			else{
//				printf("test3\n");
				ans = MOD(solve2(K - 1, N, i - 1) + solve2(K - 1, N - 1, i - 1));
			}
		}
		else{
			if(K <= i / 2){
				ans = combination(N + K - 1, N);
			}
			else if(K <= i - 1){
				ans = solve1(K, N, i);
			}
			else{
//				printf("test6\n");
				ans = solve2(K, N, i);
			}
		}
		printf("%lld\n", ans);
	}
	return 0;
}