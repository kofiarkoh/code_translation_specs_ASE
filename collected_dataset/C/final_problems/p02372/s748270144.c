// AOJ GRL_5_B Height of a Tree
// 2019.3.9 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

int n;
typedef struct { int to, w; } T;
T *tbl[10005]; int hi[10005];
int ans[10005];

typedef struct { int id, w; } Q;
Q q[10005]; int end;
char visited[10005];
int max_id, max_w;

void bfs(int id)
{
	int i, s, t, w, ww;

	memset(visited, 0, n*sizeof(visited[0]));
	q[0].id = id, q[0].w = 0, end = 1, visited[id] = 1, max_w = 0;
	while (end) {
		s = q[--end].id, w = q[end].w;
		for (i = 0; i < hi[s]; i++) {
			t = tbl[s][i].to;
			if (visited[t]) continue;
			visited[t] = 1, ww = w + tbl[s][i].w;
			if (ww > ans[t]) ans[t] = ww;
			q[end].id = t, q[end++].w = ww;
			if (ww > max_w) max_w = ww, max_id = t;
		}
	}
}

int main()
{
	int i, k, s, t, w;
	int *memo, sz;

	n = in();
	memo = malloc(n * 3 * sizeof(int));
	sz = 0; for (i = 0; i < n-1; i++) {
		memo[sz++] = s = in();
		memo[sz++] = t = in();
		memo[sz++] = in();
		hi[s]++, hi[t]++;
	}
	for (i = 0; i < n; i++) tbl[i] = malloc(hi[i] * sizeof(T));
	memset(hi, 0, n*sizeof(hi[0]));
	i = 0; while (i < sz) {
		s = memo[i++], t = memo[i++], w = memo[i++];
		k = hi[s]++, tbl[s][k].to = t, tbl[s][k].w = w;
		k = hi[t]++, tbl[t][k].to = s, tbl[t][k].w = w;
	}
	free(memo);

	bfs(0);
	bfs(max_id);
	bfs(max_id);
	for (i = 0; i < n; i++) out(ans[i]);
	return 0;
}
