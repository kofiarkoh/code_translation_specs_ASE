#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p (int)(1e9 + 7)

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

signed main(){
	int N, M, i, j, A, B, Ac, ans = 1;
	scanf("%lld%lld", &N, &M);
	int *a = (int *)malloc(sizeof(int) * M);
	int *b = (int *)malloc(sizeof(int) * M);
	for(i = 0; i < M; i++){
		scanf("%lld%lld", &a[i], &b[i]);
		a[i]--;
		b[i]--;
	}
//	printf("test1\n");

	int *E = (int *)malloc(sizeof(int) * (1 << N));
	for(i = 0; i < (1 << N); i++){
		E[i] = 0;
		for(j = 0; j < M; j++){
			E[i] += ((((i | (1 << a[j])) == i) ? 1 : 0) ^ (((i | (1 << b[j])) == i) ? 1 : 0));
		}
//		printf("E[%lld] = %lld\n", i, E[i]);
	}
//	printf("test2\n");

	int *pow2inv = (int *)malloc(sizeof(int) * (M + 1));
	pow2inv[0] = 1;
	for(i = 1; i <= M; i++){
		pow2inv[i] = MOD(pow2inv[i - 1] * ((p + 1) / 2));
	}
//	printf("test3\n");

	int *P = (int *)malloc(sizeof(int) * (1 << N));
	P[1] = 1;
	for(A = 0; A < (1 << N); A++){
		if((A & 1) != 0){
//			printf("A = %lld\n", A);
			P[A] = 1;
			for(i = ((A - 1) & A); i != A; i = ((i - 1) & A)){
				if((i | 1) == i){
//					printf("i = %lld\n", i);
					P[A] = MOD(P[A] - P[i] * pow2inv[(E[i] + E[A - i] - E[A]) / 2]);
				}
			}
		}
		else{
			P[A] = 0;
		}
//		printf("P[%lld] = %lld\n", A, P[A]);
	}
//	printf("test4\n");

	int *Q = (int *)malloc(sizeof(int) * (1 << N));
	Q[2] = 1;
	for(B = 0; B < (1 << N); B++){
		if((B & 2) != 0){
			Q[B] = 1;
			for(i = ((B - 1) & B); i != B; i = ((i - 1) & B)){
				if((i | 2) == i){
					Q[B] = MOD(Q[B] - Q[i] * pow2inv[(E[i] + E[B - i] - E[B]) / 2]);
				}
			}
		}
	}
//	printf("test5\n");

	for(A = 0; A < (1 << N); A++){
		Ac = (1 << N) - 1 - A;
		B = Ac;
		do{
			if((A & 1) != 0 && (B & 2) != 0 && E[A] + E[B] - E[A + B] == 0){
				ans = MOD(ans - MOD(P[A] * Q[B]) * pow2inv[E[A] + E[B]]);
			}
			B = ((B - 1) & Ac);
		}while(B != Ac);
	}
	printf("%lld\n", MOD(ans * power(2, M)));
	return 0;
}