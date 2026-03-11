#include <stdio.h>
#include <stdlib.h>

int max(int a, int b){
	return a >= b ? a : b;
}

int next(int R, int *es, int *emaxs){
	int i;
	for(i = 0; i < R; i++){
		if(es[i] < emaxs[i]){
			es[i]++;
			return 1;
		}
		else{
			es[i] = 0;
		}
	}
	return 0;
}

int is_covered(int R, int *m_facts, int *es){
	int i;
	for(i = 0; i < R; i++){
		if(m_facts[i] > es[i]){
			return 0;
		}
	}
	return 1;
}

int main(){
	int N, K, R, Q, i, j, now, ans = 0;
	scanf("%d", &Q);
	int *m = (int *)malloc(sizeof(int) * Q);
	int *x = (int *)malloc(sizeof(int) * Q);
	for(i = 0, N = 0; i < Q; i++){
		scanf("%d%d", &m[i], &x[i]);
		N = max(N, m[i]);
	}
	int *prime_num = (int *)malloc(sizeof(int) * (N + 1));
	for(i = 0; i <= N; i++){
		prime_num[i] = 0;
	}
	prime_num[0] = -1;
	prime_num[1] = -1;
	for(i = 2, K = 0; i <= N; i++){
		if(prime_num[i] == 0){
			prime_num[i] = K;
			K++;
			for(j = 2; i * j <= N; j++){
				prime_num[i * j] = -1;
			}
		}
	}
//	printf("N = %d\n", N);
/*	for(i = 0; i <= N; i++){
		printf("prime_num[%d] = %d\n", i, prime_num[i]);
	}
	printf("\n");
*/	int *primes = (int *)malloc(sizeof(int) * K);
	int *emaxs = (int *)malloc(sizeof(int) * K);
	for(i = 0; i <= N; i++){
		if(prime_num[i] >= 0){
			primes[prime_num[i]] = i;
		}
	}
	for(i = 0; i < K; i++){
		for(emaxs[i] = -1, now = 1; now <= N; emaxs[i]++, now *= primes[i]){}
	}
//	printf("K = %d\n", K);
/*	for(i = 0; i < K; i++){
		printf("primes[%d] = %d\n", i, primes[i]);
	}
	printf("\n");
	for(i = 0; i < K; i++){
		printf("emaxs[%d] = %d\n", i, emaxs[i]);
	}
	printf("\n");
*/	for(R = 0; emaxs[R] > 1; R++){}
//	printf("R = %d\n", R);

	int **m_facts = (int **)malloc(sizeof(int *) * Q);
	int *max_fact = (int *)malloc(sizeof(int) * Q);
	for(i = 0; i < Q; i++){
		m_facts[i] = (int *)malloc(sizeof(int) * K);
		for(j = 0; j < K; j++){
			m_facts[i][j] = 0;
		}
		for(j = 0; m[i] > 1; j++){
			if(m[i] % primes[j] == 0){
				m_facts[i][j]++;
				m[i] /= primes[j];
				max_fact[i] = j;
				j--;
			}
		}
//		printf("i = %d\n", i);
/*		for(j = 0; j < K; j++){
			printf("%d ", m_facts[i][j]);
		}
		printf("\n");
		printf("max_fact[%d] = %d\n\n", i, max_fact[i]);
*/	}

	int *es = (int *)malloc(sizeof(int) * R);
	for(i = 0; i < R; i++){
		es[i] = 0;
	}
	int *A = (int *)malloc(sizeof(int) * K);
	do{
/*		printf("es:");
		for(i = 0; i < R; i++){
			printf("%d ", es[i]);
		}
		printf("\n");
*/		now = 0;
		for(i = R; i < K; i++){
			A[i] = 0;
		}
		for(i = 0; i < Q; i++){
			if(is_covered(R, m_facts[i], es) == 1){
				if(max_fact[i] < R){
					now += x[i];
				}
				else{
					A[max_fact[i]] += x[i];
				}
			}
		}
		for(i = R; i < K; i++){
			now += max(0, A[i]);
		}
		ans = max(ans, now);
	}while(next(R, es, emaxs) == 1);
	printf("%d\n", ans);
	return 0;
}