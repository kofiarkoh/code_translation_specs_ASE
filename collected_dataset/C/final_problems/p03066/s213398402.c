#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p 998244353

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

int inverse(int a){
	return power(a, p - 2);
}

int *fact, *fact_inv;

int comb(int N, int k){
	if(k < 0 || N < k){
		return 0;
	}
	else{
		return MOD(fact[N] * MOD(fact_inv[k] * fact_inv[N - k]));
	}
}

signed main(){
	int N, X, i, j, K, x;
	scanf("%lld%lld", &N, &X);
	int *pow2 = (int *)malloc(sizeof(int) * (N + 1));
	pow2[0] = 1;
	for(i = 1; i <= N; i++){
		pow2[i] = MOD(2 * pow2[i - 1]);
	}
	fact = (int *)malloc(sizeof(int) * (N + 1));
	fact[0] = 1;
	for(i = 1; i <= N; i++){
		fact[i] = MOD(fact[i - 1] * i);
	}
	fact_inv = (int *)malloc(sizeof(int) * (N + 1));
	for(i = 0; i <= N; i++){
		fact_inv[i] = inverse(fact[i]);
	}
	int ans = 0, now;
	for(K = 0; K <= N; K++){
//		printf("K = %lld\n", K);
		now = 0;
		for(x = K; x < X && x <= 2 * K; x++){
			now = MOD(now + comb(K, x - K));
		}
//		printf("now1 = %lld\n", now);
		if(X % 2 == 1 && K >= X - 1 && 2 * K > X){
			now++;
		}
		for(j = 1; 2 * j < K; j++){
			now = MOD(now + comb(K - 2 * j, X - 1 - K));
		}
//		printf("now = %lld\n", now);
		ans = MOD(ans + comb(N, K) * now);
	}
	printf("%lld\n", ans);
	return 0;
}