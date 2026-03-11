#include <stdio.h>
#include <stdlib.h>
#define int long long

signed main(){
	int N, M, K, i, j, k, count, ans = 0;
	scanf("%lld%lld", &N, &K);
	int *a = (int *)malloc(sizeof(int) * (N + 1));
	int *s = (int *)malloc(sizeof(int) * (N + 1));
	s[0] = 0;
	for(i = 1; i <= N; i++){
		scanf("%lld", &a[i]);
		s[i] = s[i - 1] + a[i];
	}
	M = (N * (N + 1)) / 2;
//	printf("M = %lld\n", M);
	int *b = (int *)malloc(sizeof(int) * M);
	int *arive = (int *)malloc(sizeof(int) * M);
	for(i = 0, k = 0; i < N; i++){
		for(j = i; j < N; j++){
//			printf("k = %lld\n", k);
//			printf("(i, j) = (%d, %d)\n", i, j);
			b[k] = s[j + 1] - s[i];
//			printf("b[%lld] = %lld\n", k, b[k]);
			arive[k] = 1;
			k++;
		}
	}
	for(k = 50; k >= 0; k--){
//		printf("k = %d\n", k);
		for(i = 0, count = 0; i < M; i++){
			if((b[i] | ((int)1 << k)) == b[i] && arive[i] == 1){
				count++;
			}
		}
//		printf("count = %d\n", count);
		if(count >= K){
			ans += ((int)1 << k);
		}
		for(i = 0; i < M; i++){
			if((b[i] | ans) != b[i]){
				arive[i] = 0;
			}
		}
	}
	printf("%lld\n", ans);
	return 0;
}