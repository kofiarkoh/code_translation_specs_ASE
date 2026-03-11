// AOJ 1386 Starting a Scenic Railroad Service
// 2018.2.6 bal4u

#include <stdio.h>

int a[200002], b[200002];
int imos[200002], imos1[200002], imos2[200002];

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
	int n, i, s1, s2, t, max;

	n = in();
	max = 0;
	for (i = 0; i < n; i++) {
		int _a, _b;
		a[i] = _a = in(), b[i] = _b = in();
		if (_b > max) max = _b;
		imos [_a]++, imos [_b]--;
		imos1[_a]++, imos2[_b]++;
	}

	s1 = s2 = 0;
    for (i = 1; i <= max; i++) {
        imos [i] += imos [i-1];
		if (imos[i] > s2) s2 = imos[i];
        imos1[i] += imos1[i-1];
        imos2[i] += imos2[i-1];
    }
    for (i = 0; i < n; i++) {
		t = imos1[b[i]-1] - imos2[a[i]];
		if (t > s1) s1 = t;
    }
    printf("%d %d\n", s1, s2);
	return 0;
}
