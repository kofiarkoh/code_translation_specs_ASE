// AOJ 2480 Blame Game
// 2018.3.9 bal4u

#include <stdio.h>
#include <string.h>

#define MAX 1005
short hi[MAX], to[MAX][MAX];
char  seen[MAX];
short match[MAX];

int bpm(int u)
{
	int i, v;

	for (i = 0; i < hi[u]; i++) {
		v = to[u][i];
		if (seen[v]) continue;
		seen[v] = 1;
		if (match[v] < 0 || bpm(match[v])) {
			match[u] = v, match[v] = u;
			return 1;
		}
	}
	return 0;
}

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, m, i, j, k;

	memset(match, -1, sizeof(match));

	n = in(), m = in();
	for (i = 0; i < n; i++) {
		k = in(); for (j = 0; j < k; j++) to[i][hi[i]++] = n+in()-1;
	}

	k = 0; for (i = 0; i < n; i++) {
		memset(seen, 0, n+m);
		if (bpm(i)) k++;
	}
	puts(k == m? "Bob": "Alice");
	return 0;
}

