// AOJ 2503 Project Management
// 2018.3.21 bal4u

#include <stdio.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

short hi[401], to[401][401];
short r_hi[401], rev[401][401], day[401][401];
int   dp[401];

short q[405], top, end;
short count[405];
short ord[405]; int sz;
void tsort(int n)
{
    int i, j, k;

	for (i = 0; i < n; i++) for (j = 0; j < hi[i]; j++) count[to[i][j]]++;
    top = end = 0, sz = 0;
    for (i = 0; i < n; i++) if (!count[i]) q[end++] = i;
    while (top < end) {
		i = q[top++];
		ord[sz++] = i;
        for (j = 0; j < hi[i]; j++) {
            k = to[i][j];
            if (--count[k] == 0) q[end++] = k;
        }
    }
}

int main()
{
	int n, m, a, b, c, i, j;
	
	n = in(), m = in();
	while (m--) {
		a = in(), b = in(), c = in();
		to[a][hi[a]++] = b;
		rev[b][r_hi[b]] = a, day[b][r_hi[b]++] = c;
	}
	tsort(n);

	for (j = 1; j < sz; j++) {
		b = ord[j];
		for (i = 0; i < r_hi[b]; i++) {
			a = rev[b][i];
			c = dp[a] + day[b][i];
			if (c > dp[b]) dp[b] = c;
		}
	}
	printf("%d\n", dp[n-1]);
	return 0;
}

