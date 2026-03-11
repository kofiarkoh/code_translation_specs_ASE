// AOJ 2064: Make Friendships
// 2017.11.8 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 2003
typedef struct { int v, id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;

int lookup(int v)
{
	HASH *p = hash + v % HASHSIZ;
	while (p->v) {
		if (p->v == v) return p->id;
		if (++p == hashend) p = hash;
	}
	return -1;
}

void insert(int v, int id)
{
	HASH *p = hash + v % HASHSIZ;
	while (p->v) {
		if (p->v == v) return;
		if (++p == hashend) p = hash;
	}
	p->v = v, p->id = id;
}

#define MAX 1002
int   m, n;
char  graph[MAX][MAX];
char  seen[MAX];
short matchL[MAX], matchR[MAX];

int bpm(int u)
{
  int v;

  for (v = 0; v < n; v++) if (graph[u][v]) {
    if (seen[v]) continue;
    seen[v] = 1;
    if (matchR[v] < 0 || bpm(matchR[v])) {
      matchL[u] = v;
      matchR[v] = u;
      return 1;
    }
  }
  return 0;
}

int main()
{
	int M, i, j, ans;

	while (scanf("%d", &n) && n > 0) {
		memset(hash, 0, sizeof(hash));
		memset(graph, 0, sizeof(graph));
		scanf("%d", &M);
		m = 0; while (M--) {
			scanf("%d", &j);
			insert(j, m++);
		}
		for (i = 0; i < n; i++) {
			scanf("%d", &M);
			while (M--) {
				scanf("%d", &j);
				if ((j = lookup(j)) >= 0) graph[j][i] = 1;
			}
		}

	    memset(matchL, -1, m*sizeof(matchL[0]));
		memset(matchR, -1, n*sizeof(matchR[0]));

		ans = 0;
		for (i = 0; i < m; i++) {
			memset(seen, 0, n);
			if (bpm(i)) ans++;
		}
		printf("%d\n", ans);
	}
	return 0;
}