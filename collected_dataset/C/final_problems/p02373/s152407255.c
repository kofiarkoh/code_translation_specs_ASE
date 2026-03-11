// AOJ GRL_5_C Lowest Common Ancestor
// 2018.5.5 bal4u

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

	if (!n) pc('0');
	else {
//		if (n < 0) pc('-'), n = -n;
		i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}


#define INF 0x30303030

// セグメント木の初期化
int seg[1<<19][2]; int sz;

void segtree(int n, int init_val)
{
	int i;

	sz = 2; while (sz < n) sz <<= 1;
	n = sz << 1;
//	memset(seg, init_val, n << 2);
	for (i = 0; i < n; i++) seg[i][0] = init_val;
}

// RMQのインデックス部分を一括して更新
void update()
{
	int i, k, n, p;

	n = sz;	while (n > 0) {
		for (p = n-1, i = 0; i < n; i+=2, p+=2) {
			k = p >> 1;
			if (seg[p][0] <= seg[p+1][0]) {
				seg[k][0] = seg[p  ][0], seg[k][1] = seg[p  ][1];
			} else {
				seg[k][0] = seg[p+1][0], seg[k][1] = seg[p+1][1];
			}
		}
		n >>= 1;
	}
}


int query2(int a, int b, int k, int l, int r, int *id) 
{
	int m, lmin, rmin;
	int lid, rid;

	if (r <= a || b <= l) { *id = 0; return INF; }
	if (a <= l && r <= b) { *id = seg[k][1]; return seg[k][0]; }
	k <<= 1, m = (l+r) >> 1;
    lmin = query2(a, b, k+1, l, m, &lid);
    rmin = query2(a, b, k+2, m, r, &rid);
	if (lmin > rmin) lmin = rmin, lid = rid;
	*id = lid;
    return lmin; 
}

// 空間 [a, b] 内の最小値
int range_min_query(int a, int b)
{
	int id;
	query2(a, b+1, 0, 0, sz, &id);
	return id;
}


// LCA
#define MAX_V  100005
int hi[MAX_V], *to[MAX_V];
int root;
int vs[MAX_V << 1];
int depth[MAX_V << 1];
int id[MAX_V];

void dfs(int v, int p, int d, int *k)
{
	int i;

	id[v] = *k;
	vs[*k] = v;
	depth[(*k)++] = d;
	for (i = 0; i < hi[v]; i++) {
		if (to[v][i] != p) {
			dfs(to[v][i], v, d+1, k);
			vs[*k] = v;
			depth[(*k)++] = d;
		}
	}
}

int lca(int u, int v)
{
	int min, max;
	if (id[u] < id[v]) min = id[u], max = id[v];
	else               min = id[v], max = id[u];
	return vs[range_min_query(min, max)];
}

int main()
{
	int n, q, i, j, k, u, v;
	int *memo;

	n = in();
	memo = malloc(n<<3);
	i = 0; for (u = 0; u < n; u++) {
		k = in();
		for (j = 0; j < k; j++) {
			memo[i++] = u, memo[i++] = v = in();
			hi[u]++, hi[v]++;
		}
	}

	for (u = 0; u < n; u++) if (hi[u]) to[u] = malloc(hi[u]<<2);
	memset(hi, 0, n<<2);
	j = 0; while (j < i) {
		u = memo[j++], v = memo[j++];
		to[u][hi[u]++] = v, to[v][hi[v]++] = u;
	}

	root = 0, k = 0;
	dfs(root, -1, 0, &k);

	n = (n<<1)-1;
	segtree(n, INF);

	j = sz-1;
	for (u = 0; u < n; u++) seg[j][0] = depth[u], seg[j++][1] = u;
	update();

	q = in();
	while (q--) {
		u = in(), v = in();
		out(lca(u, v));
	}
	return 0;
}
