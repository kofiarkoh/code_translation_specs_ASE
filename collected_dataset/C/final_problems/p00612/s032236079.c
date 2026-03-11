// AOJ 1026 Hedro's Hexahedron
// 2018.2.2 bal4u@uu

#include <stdio.h>

int main()
{
	int n;
	long long i, k, s, t, ans;

	while(scanf("%d", &n) && n){
		ans = n << 3;
        s = t = n >> 1;
        for(i = 1, k = 2; i*i < s; i++, k+=2) t += (n+k-1)/k;
        t <<= 1, t -= i*i;
        if (n & 1) t++;
        printf("%lld\n", ans + (t << 3));
	}
	return 0;
}
