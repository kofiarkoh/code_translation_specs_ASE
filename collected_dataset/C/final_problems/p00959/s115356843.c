// AOJ 1380 Medical Checkup
// 2018.2.6 bal4u

#include <stdio.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, t, h, b, i, p;

	n = in(), t = in();
	b = 0, p = 0;
	for (i = 0; i < n; i++) {
		h = in();
		if (p < h) p = h;
		printf("%d\n", 1+(t-b)/p + ((t-b)%p >= h));
		b += h;
		if (b >= t) break;
	}
	for (; i < n; i++) puts("1");
	return 0;
}
