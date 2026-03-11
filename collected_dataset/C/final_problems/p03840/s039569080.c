#include <stdio.h>
#define int long long

int min(int a, int b){
	return a <= b ? a : b;
}

signed main(){
	int aI, aO, aT, aJ, aL, aS, aZ, ans, minIJL;
	scanf("%lld%lld%lld%lld%lld%lld%lld", &aI, &aO, &aT, &aJ, &aL, &aS, &aZ);
	ans = aO;
	minIJL = min(aI, min(aJ, aL));
	aI -= minIJL;
	aJ -= minIJL;
	aL -= minIJL;
	ans += 3 * minIJL;
	ans += (aI / 2) * 2;
	ans += (aJ / 2) * 2;
	ans += (aL / 2) * 2;
	printf("%lld\n", ans);
	return 0;
}