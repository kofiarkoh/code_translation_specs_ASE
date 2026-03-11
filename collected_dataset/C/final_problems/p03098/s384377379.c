#include <stdio.h>
#include <stdlib.h>

int N;

int *inverse(int *a){
	int i;
	int *a_inv = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		a_inv[a[i]] = i;
	}
	return a_inv;
}

int *mul(int *a, int *b){
	int i;
	int *c = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		c[i] = a[b[i]];
	}
	return c;
}

int *power(int *a, int n){
	if(n == 0){
		int i;
		int *ans = (int *)malloc(sizeof(int) * N);
		for(i = 0; i < N; i++){
			ans[i] = i;
		}
		return ans;
	}
	else if(n % 2 == 0){
		return power(mul(a, a), n / 2);
	}
	else{
		return mul(a, power(a, n - 1));
	}
}

int main(){
	int K, i;
	scanf("%d%d", &N, &K);
	K--;
	int *p = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &p[i]);
		p[i]--;
	}
	int *q = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &q[i]);
		q[i]--;
	}
	int **ans_small = (int **)malloc(sizeof(int *) * 6);
	ans_small[0] = p;
	ans_small[1] = q;
	for(i = 2; i < 6; i++){
		ans_small[i] = mul(ans_small[i - 1], inverse(ans_small[i - 2]));
	}
/*	for(i = 0; i < 6; i++){
		printf("ans_small[%d]\n", i);
		int j;
		for(j = 0; j < N; j++){
			printf("%d ", ans_small[i][j]);
		}
		printf("\n");
	}
*/	int *qPQp = mul(mul(q, inverse(p)), mul(inverse(q), p));
	int *ans = mul(mul(power(qPQp, K / 6), ans_small[K % 6]), power(inverse(qPQp), K / 6));
	for(i = 0; i < N; i++){
		printf("%d ", ans[i] + 1);
	}
	printf("\n");
	return 0;
}