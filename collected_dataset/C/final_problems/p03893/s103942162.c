#include <stdio.h>
#define int long long

signed main(){
	int X, i, N;
	scanf("%lld", &X);
	for(i = 0, N = 2; i < X; i++){
		N = N + (N + 1) + 1;
	}
	printf("%lld\n", N);
	return 0;
}