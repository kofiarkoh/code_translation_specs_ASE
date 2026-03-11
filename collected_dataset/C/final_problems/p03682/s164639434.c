#include <stdio.h>
#include <stdlib.h>
typedef long long ll;
typedef struct {
	ll point;
	ll node;
} Pair;

typedef struct {
	int to, from;
	ll cost;
} Edge;

int parent[100005];

int root(int a);
void unite(int a, int b);
int Pair_cmp(const Pair *, const Pair *);
int Edge_cmp(const Edge *, const Edge *);

ll n;
Pair x[100005];
Pair y[100005];
Edge e[2 * 100005];

int main()
{
	for (ll i = 0; i < sizeof(parent) / sizeof(parent[0]); i++) {
		parent[i] = -1;
	}

	int e_num = 0;
	ll ans = 0;

	scanf("%lld", &n);
	for (ll i = 0; i < n; i++) {
		scanf("%lld %lld", &x[i].point, &y[i].point);

		x[i].node = y[i].node = i + 1;
	}

	qsort(x, n, sizeof(Pair), (int (*)(const void *, const void *))Pair_cmp);
	qsort(y, n, sizeof(Pair), (int (*)(const void *, const void *))Pair_cmp);

	for (ll i = 0; i < n - 1; i++) {
		e[e_num].to = x[i].node;
		e[e_num].from = x[i + 1].node;
		e[e_num].cost = x[i + 1].point - x[i].point;

		e_num++;

		e[e_num].to = y[i].node;
		e[e_num].from = y[i + 1].node;
		e[e_num].cost = y[i + 1].point - y[i].point;

		e_num++;
	}

	qsort(e, e_num, sizeof(Edge), (int (*)(const void *, const void *))Edge_cmp);

	for (ll i = 0; i < e_num; i++) {

		if (root(e[i].to) != root(e[i].from)) {
			unite(e[i].to, e[i].from);
			ans = ans + e[i].cost;
		}

	}

	printf("%lld\n", ans);

}

int root(int a)
{
	if (parent[a] < 0) {
		return a;
	} else {
		return parent[a] = root(parent[a]);
	}
}

void unite(int a, int b)
{
	a = root(a);
	b = root(b);

	if (a != b) {
		parent[a] += parent[b];
		parent[b] = a;
	}

	return;
}

int Pair_cmp(const Pair *x, const Pair *y)
{
	if (x->point > y->point) {
		return 1;
	} else if (x->point == y->point) {
		return 0;
	} else {
		return -1;
	}
}

int Edge_cmp(const Edge *x, const Edge *y)
{
	if (x->cost > y->cost) {
		return 1;
	} else if(x->cost == y->cost) {
		return 0;
	} else {
		return -1;
	}
}
