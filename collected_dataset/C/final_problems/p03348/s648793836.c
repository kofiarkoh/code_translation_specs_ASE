#include <stdio.h>

typedef struct List {
	struct List *next;
	int v;
} list;

int main()
{
	int i, u, w, N;
	scanf("%d", &N);
	list *adj[101], d[200];
	for (i = 1; i <= N; i++) adj[i] = NULL;
	for (i = 0; i < N; i++) {
		scanf("%d %d", &u, &w);
		d[i*2].v = w;
		d[i*2+1].v = u;
		d[i*2].next = adj[u];
		d[i*2+1].next = adj[w];
		adj[u] = &(d[i*2]);
		adj[w] = &(d[i*2+1]);
	}
	
	list *p;
	int dist[101] = {0, -1}, par[101] = {}, q[101] = {1}, head, tail;
	for (head = 0, tail = 1; head < tail; head++) {
		for (p = adj[q[head]]; p != NULL; p = p->next) {
			if (dist[p->v] == 0) {
				dist[p->v] = -1;
				q[tail++] = p->v;
			}
		}
	}
	q[0] = q[tail-1];
	dist[q[tail-1]] = 0;
	par[q[tail-1]] = q[tail-1];
	for (head = 0, tail = 1; head < tail; head++) {
		for (p = adj[q[head]]; p != NULL; p = p->next) {
			if (dist[p->v] == -1) {
				dist[p->v] = dist[q[head]] + 1;
				par[p->v] = q[head];
				q[tail++] = p->v;
			}
		}
	}
	
	int j, k, r, s, t, diam = dist[q[tail-1]], h[101];
	long long ans, tmp, deg[101] = {};
	if (diam % 2 == 0) {
		for (r = q[tail-1]; dist[r] > diam / 2; r = par[r]);
		for (i = 1; i <= N; i++) dist[i] = -1;
		q[0] = r;
		dist[r] = 0;
		for (head = 0, tail = 1; head < tail; head++) {
			for (p = adj[q[head]], tmp = 0; p != NULL; p = p->next) {
				if (dist[p->v] == -1) {
					dist[p->v] = dist[q[head]] + 1;
					q[tail++] = p->v;
					tmp++;
				}
			}
			if (tmp > deg[dist[q[head]]]) deg[dist[q[head]]] = tmp;
		}
		for (i = 0, ans = 1; i < diam / 2; i++) ans *= deg[i];
		
		for (k = 1; dist[q[k]] == 1; k++) h[k] = q[k];
		for (j = 1; j < k; j++) {
			for (i = 1; i <= N; i++) dist[i] = -1;
			for (i = 0; i <= diam / 2; i++) deg[i] = 0;
			q[0] = r;
			q[1] = h[j];
			dist[r] = 0;
			dist[h[j]] = 0;
			for (head = 0, tail = 2; head < tail; head++) {
				for (p = adj[q[head]], tmp = 0; p != NULL; p = p->next) {
					if (dist[p->v] == -1) {
						dist[p->v] = dist[q[head]] + 1;
						q[tail++] = p->v;
						tmp++;
					}
				}
				if (tmp > deg[dist[q[head]]]) deg[dist[q[head]]] = tmp;
			}
			for (i = 0, tmp = 2; i < diam / 2; i++) tmp *= deg[i];
			if (tmp < ans) ans = tmp;
		}
	} else {
		for (s = q[tail-1]; dist[s] > (diam + 1) / 2; s = par[s]);
		t = par[s];
		for (i = 1; i <= N; i++) dist[i] = -1;
		q[0] = s;
		q[1] = t;
		dist[s] = 0;
		dist[t] = 0;
		for (head = 0, tail = 2; head < tail; head++) {
			for (p = adj[q[head]], tmp = 0; p != NULL; p = p->next) {
				if (dist[p->v] == -1) {
					dist[p->v] = dist[q[head]] + 1;
					q[tail++] = p->v;
					tmp++;
				}
			}
			if (tmp > deg[dist[q[head]]]) deg[dist[q[head]]] = tmp;
		}
		for (i = 0, ans = 2; i < diam / 2; i++) ans *= deg[i];
	}
	
	printf("%d %lld\n", diam / 2 + 1, ans);
	fflush(stdout);
	return 0;
}