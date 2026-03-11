#include <stdio.h>
#include <stdlib.h>
#define int long long

signed main(){
	int N, i, j, sum;
	scanf("%lld", &N);
	int *a = (int *)malloc(sizeof(int) * (N + 1));
	for(i = 1; i <= N; i++){
		scanf("%lld", &a[i]);
	}
	for(i = N; i > 0; i--){
		if(a[i] < 0){
			sum = 0;
			for(j = i; j <= N; j += i){
				sum += a[j];
			}
			if(sum <= 0){
				for(j = i; j <= N; j += i){
					a[j] = 0;
				}
			}
		}
	}
	sum = 0;
	for(i = 1; i <= N; i++){
		sum += a[i];
	}
	printf("%lld\n", sum);
	return 0;
}