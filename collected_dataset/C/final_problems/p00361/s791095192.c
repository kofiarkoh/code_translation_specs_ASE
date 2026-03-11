// AOJ 0366: Road Improvement
// 2019.2.25 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

// 強連結成分への分解モジュール
#define MAX_V 10002
int  V;    // 頂点数
int  *to[MAX_V], hi[MAX_V];
int  *r_to[MAX_V], r_hi[MAX_V];
int  vs[MAX_V], vsz;
char used[MAX_V];   // 作業用
int  cmp[MAX_V];    // 属する強連結成分のトポロジカル順序

void add_edge(int _from, int _to)
{
	to[_from][hi[_from]++] = _to;
	r_to[_to][r_hi[_to]++] = _from;
}

void dfs(int v)
{
	int i;
	used[v] = 1;
	for (i = 0; i < hi[v]; i++) {
		if (!used[to[v][i]]) dfs(to[v][i]);
	}
	vs[vsz++] = v;
}

void rdfs(int v, int k)
{
	int i;
	used[v] = 1;
	cmp[v] = k;
	for (i = 0; i < r_hi[v]; i++) {
		if (!used[r_to[v][i]]) rdfs(r_to[v][i], k);
	}
}

// 強連結成分への分解
int scc()
{
	int v, i, k;
	memset(used, 0, V);
	vsz = 0;
	for (v = 0; v < V; v++) if (!used[v]) dfs(v);
	memset(used, 0, V);
	k = 0;
	for (i = vsz - 1; i >= 0; i--) {
		if (!used[vs[i]]) rdfs(vs[i], k++);
	}

	// 同一強連結成分グループは cmp[v]が同一
	return k;		// 得られた強連結成分の数
}

int main()
{
	int i, j, E, s, t, group;
	int *memo, sz;

	V = in(), E = in();  // V:頂点数, E:エッジ数
	memo = malloc(E * 2 * sizeof(int));
	sz = 0; for (i = 0; i < E; i++) {
		memo[sz++] = s = in(), memo[sz++] = t = in();
		hi[s]++, r_hi[t]++;
	}
	for (i = 0; i < V; i++) {
		if (hi[i])   to[i] = malloc(hi[i] * sizeof(int));
		if (r_hi[i]) r_to[i] = malloc(r_hi[i] * sizeof(int));
	}
	memset(hi, 0, V * sizeof(int));
	memset(r_hi, 0, V * sizeof(int));

	i = 0; while (i < sz) {
		s = memo[i++], t = memo[i++];
		add_edge(s, t);
	}
	free(memo);

	group = scc(); // 強連結成分のグループ数
	if (group == 1) puts("0"); // 強連結成分がひとつなら道路の増設は必要なし
	else{
		char *inCnt, *outCnt; // 各強連結成分グループの入次数、出次数
		int ord, inSum, outSum;

		inCnt = malloc(group), outCnt = malloc(group);
		memset(inCnt, 0, group), memset(outCnt, 0, group);  // リセット

		for (i = 0; i < V; i++) {  // すべての頂点について
			ord = cmp[i];   // 頂点 i の強連結成分のグループ番号
			for (j = 0; j < hi[i]; j++) {  // 頂点 i から出ていくエッジについて
				if (cmp[to[i][j]] != ord) outCnt[ord] = 1;
			}
			for (j = 0; j < r_hi[i]; j++) {  // 頂点 i に入ってくるエッジについて
				if (cmp[r_to[i][j]] != ord) inCnt[ord] = 1;
			}
		}

		inSum = outSum = 0;
		for (i = 0; i < group; i++) {
			if (inCnt[i] == 0) inSum++;
			if (outCnt[i] == 0) outSum++;
		}
		printf("%d\n", inSum >= outSum ? inSum : outSum);
//		free(inCnt), free(outCnt);   // メモリ開放
	}
	return 0;
}
