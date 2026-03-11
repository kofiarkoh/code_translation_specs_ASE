// AOJ 1595 Traffic Tree
// 2018.5.4 bal4u

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
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
{
	int i;
	char ob[20];

	if (!n) { pc('0'); return; }
	if (n < 0) pc('-'), n = -n;
	i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
	while (i--) pc(ob[i]);
	pc('\n');
}

#define INF   0x10101010
#define MAX_N 100002

typedef struct { int s, d; } QUE;
QUE que[MAX_N*2]; int top, end;
int n;
int hi[MAX_N], *to[MAX_N];
int dist_a[MAX_N], dist_b[MAX_N];

void bfs(int s, int *dist)
{
	int i, t, d;

	memset(dist, INF, n<<2);
	que[0].s = s, que[0].d = 0, dist[s] = 0, top = 0, end = 1;
	while (top != end) {
		s = que[top].s, d = que[top++].d;
		if (dist[s] < d) continue;
		for (i = 0; i < hi[s]; i++) {
			t = to[s][i];
			if (dist[t] == INF) {
				dist[t] = d+1, que[end].s = t, que[end++].d = d+1;
			}
		}
	}
}

void calc()
{
	int i, max;
	int node_a, node_b;		// 木の直径を表す２つのノード
	
	bfs(0, dist_a);
	max = 0;
	for (i = 0; i < n; i++) {
		if (dist_a[i] > max) node_a = i, max = dist_a[i];
	}

	bfs(node_a, dist_a);
	max = 0;
	for (i = 0; i < n; i++) {
		if (dist_a[i] > max) node_b = i, max = dist_a[i];
	}

	bfs(node_b, dist_b);
}

int main()
{
	int s, t, i, j, ans;
	int *memo;

	n = in();
	memo = malloc(n<<3);
	j = 0; for (i = 1; i < n; i++) {
		memo[j++] = s = in()-1;
		memo[j++] = t = in()-1;
		hi[s]++, hi[t]++;
	}
	for (i = 0; i < n; i++) if (hi[i]) to[i] = malloc(hi[i]<<2);
	memset(hi, 0, n<<2);
	j = 0; for (i = 1; i < n; i++) {
		s = memo[j++], t = memo[j++];
		to[s][hi[s]++] = t;
		to[t][hi[t]++] = s;
	}
	free(memo);

	calc();

	// (t=)辺の数*2 - (ans=)頂点iを根とした根付き木の深さ
	t = (n-1)<<1;		// tは再定義
	for (i = 0; i < n; i++) {
		ans = dist_a[i];
		if (ans < dist_b[i]) ans = dist_b[i];
		out(t-ans);
	}
	return 0;
}

