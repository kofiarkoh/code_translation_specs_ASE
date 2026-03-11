/*
cat <<EOF >mistaken-paste
*/

#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"
#define _USE_MATH_DEFINES

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

#define BIG 2000000007
#define VERYBIG 200000000000007LL

#define MOD 1000000007
typedef uint64_t ull;
typedef  int64_t sll;

#define N_MAX 2000
#define M_MAX 200000

#ifdef __cplusplus
#include <queue>
#include <stack>
// #include <tuple>
#include <set>
#include <map>

// using namespace std; // HELL
using std::queue;
using std::priority_queue;
using std::stack;
// using std::tuple;
using std::set;
using std::map;
using std::vector;
using std::greater;
using std::pair;

#endif

typedef struct {
	int32_t a;
	int32_t b;
} hw;

typedef struct {
	sll a;
	sll b;
} hwll;

typedef struct {
	hwll a;
	hwll b;
} linell;


typedef struct {
	ull s;
	ull t;
	int32_t c;
} struct_a;

typedef struct {
	int32_t from;
	int32_t to;
	sll cost;
} struct_b;



const hw vector8[8] = {
	{-1, -1},
	{-1,  0},
	{-1, +1},
	{ 0, -1},
	{ 0, +1},
	{+1, -1},
	{+1,  0},
	{+1, +1}
};

ull n, m;
ull h, w;
ull k;
ull q;
ull vua, vub, vuc, vud, vue, vuf;
sll vsa, vsb, vsc, vsd, vse, vsf;
long double vra, vrb, vrc;
double vda, vdb, vdc;
size_t slen;
size_t tlen;
char ch, dh;

ull umin (ull x, ull y) {
	return (x < y) ? x : y;
}

ull umax (ull x, ull y) {
	return (x > y) ? x : y;
}

sll smin (sll x, sll y) {
	return (x < y) ? x : y;
}

sll smax (sll x, sll y) {
	return (x > y) ? x : y;
}

ull gcd (ull x, ull y) {
	if (x < y) {
		return gcd(y, x);
	} else if (y == 0) {
		return x;
	} else {
		return gcd(y, x % y);
	}
}

ull bitpow (ull a, ull x, ull modulo) {
	ull result = 1;
	while (x) {
		if (x & 1) {
			result *= a;
			result %= modulo;
		}
		x /= 2;
		a = (a * a) % modulo;
	}
	return result;
}

ull divide (ull a, ull b, ull modulo) {
	return (a * bitpow(b, modulo - 2, modulo)) % modulo;
}

ull udiff (ull a, ull b) {
	if (a >= b) {
		return a - b;
	} else {
		return b - a;
	}
}

sll sdiff (sll a, sll b) {
	if (a >= b) {
		return a - b;
	} else {
		return b - a;
	}
}

void printUquotient (ull left, ull right) {
	const int32_t digits = 20;

	printf("%llu.", left / right);
	left %= right;
	for (int32_t i = 0; i < digits; i++) {
		left *= 10;
		printf("%1d", left / right);
		left %= right;
	}
	puts("");

	return;
}

void printSquotient (sll left, sll right) {
	if (left * right < 0) putchar('-');
	printUquotient(sdiff(left, 0), sdiff(right, 0));

	return;
}

int bitcount (ull n) {
	int result = 0;
	while (n) {
		if (n & 1) result++;
		n /= 2;
	}
	return result;
}

// #ifdef __cplusplus

// typedef struct {
// 	int32_t to;
// 	sll cost;
// } edge;
// typedef pair<sll, int32_t> P;

// std::vector<edge> g[N_MAX];
// void dijk_init (ull n, struct_b arr[]) {
// 	edge x;
// 	for (int32_t i = 0; i < n; i++) {
// 		x.to = arr[i].to;
// 		x.cost = arr[i].cost;
// 		g[arr[i].from].push_back(x);
// 	}
// }

// bool dijkstra (int s, sll distance[]) {
// 	priority_queue<P, std::vector<P>, greater<P> > que; // (最短距離, 頂点番号)
// 	que.push(P(distance[s], s));

// 	bool ischanged = false;
// 	while (!que.empty()) {
// 		P p = que.top();
// 		que.pop();

// 		sll v = p.second;
// 		if (distance[v] < p.first) continue;

// 		int32_t maxsize = g[v].size();
// 		for (int32_t i = 0; i < maxsize; i++) {
// 			edge e = g[v][i];
// 			if (distance[e.to] > distance[v] + e.cost) {
// 				distance[e.to] = distance[v] + e.cost;
// 				ischanged = true;
// 				que.push(P(distance[e.to], e.to));
// 			}
// 		}
// 	}

// 	return ischanged;
// }

// #endif

sll dist[N_MAX];
struct_b path[M_MAX * 2];

// ull a[N_MAX];
// ull a[M_MAX];
// sll a[N_MAX];
ull a[N_MAX][N_MAX];
// ull a[M_MAX][M_MAX];
// sll a[N_MAX][N_MAX];
ull b[N_MAX];
// ull b[M_MAX];
// sll b[N_MAX];
// ull c[N_MAX];
// sll c[M_MAX];
// char c[N_MAX];
// char s[N_MAX + 1];
char s[N_MAX + 1][N_MAX + 1];
// char s[N_MAX + 1][M_MAX + 1];
// char t[N_MAX + 1];

ull alphabets[26];
// ull blphabets[26];
// char alphabets[26];

// ull dp[N_MAX + 1];
// sll dp[N_MAX + 1];
// ull dp[N_MAX + 1][N_MAX + 1];
// sll dp[N_MAX + 1][N_MAX + 1];
// bool dp[N_MAX + 1];
bool dp[N_MAX + 1][N_MAX + 1];
// bool dq[N_MAX];
hwll arr[N_MAX + 1];
// hwll arr[M_MAX];
hwll brr[N_MAX];

// sll dp[N_MAX + 1];
// ull color[N_MAX];
// set<sll> children[N_MAX];
// set<sll> leaf[N_MAX];

double distance (sll x1, sll y1, sll x2, sll y2) {
	double xdist2, ydist2, origindist, dist;

	xdist2 = (x1 - x2) * (x1 - x2);
	ydist2 = (y1 - y2) * (y1 - y2);
	return sqrt(xdist2 + ydist2);
}

int32_t pullcomp (const void *left, const void *right) {
	ull l = *(ull*)left;
	ull r = *(ull*)right;
	if (l < r) {
		return -1;
	}
	if (l > r) {
		return +1;
	}
	return 0;
}

int32_t phwllABcomp (const void *left, const void *right) {
	hwll l = *(hwll*)left;
	hwll r = *(hwll*)right;
	if (l.a < r.a) {
		return -1;
	}
	if (l.a > r.a) {
		return +1;
	}
	if (l.b < r.b) {
		return -1;
	}
	if (l.b > r.b) {
		return +1;
	}
	return 0;
}

ull accp (ull a, ull p) {
	if (a == 0) {
		return 0;
	}
	if (a == 1) {
		return p;
	}

	return divide((MOD + bitpow(a, p, MOD) - 1) % MOD, a - 1, MOD);
}

bool hoge (char c1, char c2, char c3, char c4) {
	bool x = true;
	if (c1 == '.') x = !x;
	if (c2 == '.') x = !x;
	if (c3 == '.') x = !x;
	if (c4 == '.') x = !x;

	return x;
}

ull piyo (sll index) {
	sll i, j;

	hwll q[N_MAX];
	ull qlen = 0;

	ull r = 0;
	for (j = 0; j <= w - 1; j++) {
		ull crop = (j < w - 1 ? a[index][j] : 0);
		// printf("%llu\n", j);
		// fflush(stdout);

		// printf("\t%llu\n", crop);
		while (qlen && q[qlen - 1].a > crop) {
			// printf("pop %llu in%llu\n", q[qlen - 1].a, q[qlen - 1].b);
			r = umax(r, (j - q[qlen - 1].b + 1) * (q[qlen - 1].a + 1));
			qlen--;
		}
		if (!qlen || q[qlen - 1].a < crop) {
			hwll adder = {crop, j};
			q[qlen++] = adder;
		}
	}

	return r;
}

ull solve () {
	sll i, j, ki;
	// ull result = 0;
	sll result = 0;
	// double result = 0;
	// ull maybe = 0;
	sll maybe = 0;
	// ull sum = 0;
	sll sum = 0;
	ull item;
	ull *dpcell;

	if (h == 2000 || w == 2000) {
		for (;;);
	}

	for (i = 0; i < h - 1; i++) {
		for (j = 0; j < w - 1; j++) {
			dp[i][j] = hoge(s[i][j], s[i][j + 1], s[i + 1][j], s[i + 1][j + 1]);
		}
	}

	for (j = 0; j < w - 1; j++) {
		sum = 0;
		for (i = 0; i < h - 1; i++) {
			if (dp[i][j]) {
				sum++;
			} else {
				sum = 0;
			}

			a[i][j] = sum;
			// printf("%llu ", sum);
			// fflush(stdout);
		}
		// puts("");
	}

	result = umax(h, w);
	for (i = 0; i < h - 1; i++) {
		result = umax(result, piyo(i));
	}

	printf("%llu\n", result);
	// printf("%.12lf\n", (double)result);
	// puts(s);

	return 0;

	success:
	puts("YES");
	// puts("Yes");
	// printf("%llu\n", result);
	// puts("0");
	// puts("First");
	return 0;

	fail:
	puts("NO");
	// puts("No");
	// puts("0");
	// puts("-1");
	// puts("-1 -1 -1");
	// puts("Second");
	return 1;
}

int32_t main (void) {
	int32_t i, j;
	int32_t x, y;

	// scanf("%lf", &vda, &vdb);
	// scanf("%lld%lld%lld%lld", &vsa, &vsb, &vsc, &vsd);
	// scanf("%llu%llu", &vua, &vub, &vuc, &vud);
	scanf("%llu%llu", &h, &w);
	// scanf("%*llu%*llu");
	// scanf("%llu", &n, &m);
	// scanf("%llu", &k, &n);
	// scanf("%llu%llu", &vua, &vub, &vuc, &vud, &vue, &vuf);
	// scanf("%lld%lld", &vsa, &vsb, &vsc);
	// scanf("%s", s);
	// scanf("%s", t);
	// scanf("%llu", &m);
	// scanf("%llu", &q);
	// for (i = 0; i < n; i++) {
	// 	scanf("%lld", &a[i]);
	// 	// scanf("%llu", &b[i]);
	// 	// a[i]--;
	// 	// b[i]--;
	// }
	// for (i = 0; i < n; i++) {
	// 	scanf("%llu", &b[i]);
	// }
	// for (i = 0; i < h; i++) {
	// 	for (j = 0; j < w; j++) {
	// 		scanf("%llu", &a[i][j]);
	// 	}
	// }
	for (i = 0; i < h; i++) {
		scanf("%s", &s[i]);
	}

	// scanf("%llu", &k);
	// scanf("%llu", &q);
	solve();

	return 0;
}
