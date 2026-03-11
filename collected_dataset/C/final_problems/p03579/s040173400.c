#include <stdio.h>
#include <stdlib.h>

#define INIT_CAP 10
#define INF 1000000000

typedef struct {
	int *elem, cap, size;
} VecInt;

typedef struct Edge {
	int to;
	struct Edge *next;
} Edge;

Edge e[100000], *adj[100000];
int e_size;
int d[100000];
int cnt[100000];
int queue[100000];
int front, rear;

void add(int from, int to) {
	e[e_size] = (Edge){to, adj[from]};
	adj[from] = e + e_size++;
}

int main(void) {
	int n, m;

	scanf("%d %d", &n, &m);

	for (int i = 0; i < m; i++) {
		int a, b;

		scanf("%d %d", &a, &b);
		a--; b--;

		add(a, b);
		add(b, a);
	}
	for (int i = 0; i < n; i++)
		d[i] = INF;

	d[0] = 0;
	queue[rear++] = 0;
	while (front != rear) {
		const int from = queue[front++];
		cnt[d[from]]++;
		for (Edge *p = adj[from]; p; p = p->next) {
			const int to = p->to;
			if (d[to] == INF) {
				d[to] = d[from] + 1;
				queue[rear++] = to;
			} else if (d[to] == d[from]) {
				printf("%lld\n", (long long)n * (n - 1) / 2 - m);
				return 0;
			}
		}
	}

	long long sum[2] = {0};
	for (int i = 0; i < 100000; i++)
		sum[i & 1] += cnt[i];

	printf("%lld\n", sum[0] * sum[1] - m);

	return 0;
}
