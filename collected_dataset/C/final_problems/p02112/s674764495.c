#include <stdio.h>
#include <stdlib.h>
#define p 360
#define inf (int)(1e9)

int max(int a, int b){
	if(a > b){
		return a;
	}
	else{
		return b;
	}
}

int min(int a, int b){
	if(a < b){
		return a;
	}
	else{
		return b;
	}
}

int MOD(int a){
	if(a >= 0){
		return a % p;
	}
	else{
		return p + a % p;
	}
}

int main(){
	int N, x, i, M, D, ans = inf, ans_sub;
	scanf("%d", &N);
	int *MD = (int *)malloc(sizeof(int) * N);
	int *S = (int *)malloc(sizeof(int) * N);
	int *V = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d%d%d%d", &M, &D, &V[i], &S[i]);
		MD[i] = 30 * (M - 1) + (D - 1);
	}
	for(x = 0; x < 360; x++){
		ans_sub = 0;
		for(i = 0; i < N; i++){
			if((MD[i] <= x && x < MD[i] + V[i]) || (MD[i] <= x + p && x + p < MD[i] + V[i])){
				ans_sub = max(ans_sub, S[i]);
			}
			else{
				ans_sub = max(ans_sub, S[i] - min(MOD(MD[i] - x), MOD(x - MD[i] - V[i] + 1)));
			}
		}
		ans = min(ans, ans_sub);
//		printf("(x, ans) = (%d, %d)\n", x, ans);
	}
	printf("%d\n", ans);
	return 0;
}