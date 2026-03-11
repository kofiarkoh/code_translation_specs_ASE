#include <stdio.h>
#include <stdlib.h>

#define getchar getchar_unlocked

#define INIT_CAP 10
#define INF 1000000000

inline int nextInt(void) {
	int ch;
	int minus, x;

	do {
		ch = getchar();
	} while (ch < '-');

	if (ch == '-') {
		minus = 1;
		ch = getchar();
	} else
		minus = 0;

	x = 0;
	while (ch >= '0' && ch <= '9') {
		x = x * 10 + (ch & 15);
		ch = getchar();
	}

	return minus ? -x : x;
}

typedef struct {
	int *elem, cap, size;
} VecInt;

VecInt adj[100000];
int d[100000];
int cnt[100000];
int queue[100000];
int front, rear;

void init(int n) {
	while (n--)
		adj[n] = (VecInt){malloc(sizeof(int) * INIT_CAP), INIT_CAP, 0};
}

void add(int from, int to) {
	if (adj[from].size == adj[from].cap) {
		adj[from].cap <<= 1;
		adj[from].elem = realloc(adj[from].elem, sizeof(int) * adj[from].cap);
	}
	adj[from].elem[adj[from].size++] = to;
}

int main(void) {
	int i;
	int n, m;

	scanf("%d %d", &n, &m);

	init(n);
	for (i = 0; i < m; i++) {
		int a, b;

		scanf("%d %d", &a, &b);
		a--; b--;

		add(a, b);
		add(b, a);
	}
	for (i = 0; i < n; i++)
		d[i] = INF;

	d[0] = 0;
	queue[rear++] = 0;
	while (front != rear) {
		const int from = queue[front++];
		cnt[d[from]]++;
		for (i = 0; i < adj[from].size; i++) {
			const int to = adj[from].elem[i];
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
	for (i = 0; i < 100000; i++)
		sum[i & 1] += cnt[i];

	printf("%lld\n", sum[0] * sum[1] - m);

	return 0;
}
