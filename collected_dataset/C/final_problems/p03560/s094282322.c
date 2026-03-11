#include <stdio.h>
#include <stdlib.h>
#define int long long

int digit_sum(int K){
	int ans;
	for(ans = 0; K > 0; K /= 10){
		ans += K % 10;
	}
	return ans;
}

int min(int a, int b){
	return a <= b ? a : b;
}

signed main(){
	int K, i, ans = (int)(1e9);
	scanf("%lld", &K);
	for(i = 1; i < (int)(1e5); i++){
		ans = min(ans, digit_sum(K * i));
	}
	printf("%lld\n", ans);
	return 0;
}