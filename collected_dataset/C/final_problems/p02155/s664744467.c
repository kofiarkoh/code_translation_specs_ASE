// AOJ 3057 First Kiss
// 2019.3.15 bal4u

#include <stdio.h>
#include <stdlib.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int main()
{
	int N, D, ans = 0;
	
	N = in(), D = in() + 1;
	while (N--) ans ^= (in() - 1) % D;
	puts(ans ? "First" : "Second");
	return 0;
}
