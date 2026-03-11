// AOJ 1026 Hedro's Hexahedron
// 2018.2.2 bal4u@uu

#include <stdio.h>

#define getchar_unlocked()  getchar()
long long in()
{
	long long n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	long long n, i, k, s, ans;

	while(n = in()){
        ans = s = n >> 1;
        for(i = 1, k = 2; i*i < s; i++, k+=2) ans += (n+k-1)/k;
        ans <<= 1, ans -= i*i;
        if (n & 1) ans++;
        printf("%lld\n", (ans + n) << 3);
	}
	return 0;
}
