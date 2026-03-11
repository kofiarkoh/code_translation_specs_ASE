#include <stdio.h>
#define int long long

signed main(){
	int N, k;
	scanf("%lld", &N);
	for(k = 1; (N + 1) % k == 0; k *= 2){}
	printf("%lld\n", k / 2);
	return 0;
}