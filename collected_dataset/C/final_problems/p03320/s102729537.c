#include <stdio.h>
#include <stdlib.h>
#define int long long

int S(int a){
	int ans = 0;
	while(a > 0){
		ans += a % 10;
		a /= 10;
	}
	return ans;
}

signed main(){
	int K, pl, ans, i, j;
	scanf("%lld", &K);
	if(K > 100){
		i = 0;
		printf("%d\n", 4 / i);
	}
	for(i = 0, pl = 1, ans = 1; i < K; ){
		for(j = i + 9; i < j && i < K; i++){
			ans += pl;
			printf("%lld\n", ans - 1);
//			printf("f = %lf\n", (double)(ans - 1) / S(ans - 1));
		}
		pl *= 10;
	}
	return 0;
}