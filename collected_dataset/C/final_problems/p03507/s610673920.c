#include <stdio.h>
#include <stdlib.h>
#define int long long
#define inf (int)(2 * 1e18)

signed main(){
	int N, K, i, sum;
	scanf("%lld%lld", &N, &K);
	int *w = (int *)malloc(sizeof(int) * N);
	int *d = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%lld%lld", &w[i], &d[i]);
	}
	int l = 0, h, r = inf;
	while(r - l > 1){
		h = (l + r) / 2;
		sum = 0;
		for(i = 0; i < N; i++){
			if(h >= w[i]){
				sum += 1 + (h - w[i]) / d[i];
			}
		}
		if(sum < K){
			l = h;
		}
		else{
			r = h;
		}
	}
	printf("%lld\n", r);
	return 0;
}