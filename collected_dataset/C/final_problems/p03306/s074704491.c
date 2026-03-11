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
#define VERYBIG 2000000000000007LL

#define MOD 1000000007
#define FOD  998244353
typedef uint64_t ull;
typedef  int64_t sll;

#define N_MAX 1000000
#define M_MAX 200000

#ifdef __cplusplus
#include <queue>
#include <stack>
#include <tuple>
#include <set>
#include <map>
#include <string>

// using namespace std; // HELL
using std::queue;
using std::priority_queue;
using std::stack;
using std::tuple;
using std::set;
using std::map;
using std::vector;
using std::greater;
using std::pair;
using std::string;

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
	sll a;
	sll b;
	sll c;
} hwllc;

typedef struct {
	hwll a;
	hwll b;
} linell;

ull n, m;
ull h, w;
ull k;
ull q;
ull vua, vub, vuc, vud, vue, vuf;
sll vsa, vsb, vsc, vsd, vse, vsf;
long double vra, vrb, vrc;
double vda, vdb, vdc;
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

#ifdef __cplusplus

bool setfind (set<ull> s, ull x) {
	return (s.find(x) != s.end());
}

#endif

// double distance (sll x1, sll y1, sll x2, sll y2) {
// 	double xdist2, ydist2, origindist, dist;

// 	xdist2 = (x1 - x2) * (x1 - x2);
// 	ydist2 = (y1 - y2) * (y1 - y2);
// 	return sqrt(xdist2 + ydist2);
// }

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

int32_t psllcomp (const void *left, const void *right) {
	sll l = *(sll*)left;
	sll r = *(sll*)right;
	if (l < r) {
		return -1;
	}
	if (l > r) {
		return +1;
	}
	return 0;
}

int32_t pcharcomp (const void *left, const void *right) {
	char l = *(char*)left;
	char r = *(char*)right;
	if (l < r) {
		return -1;
	}
	if (l > r) {
		return +1;
	}
	return 0;
}

int32_t pstrcomp (const void *left, const void *right) {
	char* l = *(char**)left;
	char* r = *(char**)right;

	return strcmp(l, r);
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

int32_t phwllREVcomp (const void *left, const void *right) {
	hwll l = *(hwll*)left;
	hwll r = *(hwll*)right;
	if (l.b < r.b) {
		return -1;
	}
	if (l.b > r.b) {
		return +1;
	}
	if (l.a < r.a) {
		return -1;
	}
	if (l.a > r.a) {
		return +1;
	}
	return 0;
}

int32_t ptriplecomp (const void *left, const void *right) {
	hwllc l = *(hwllc*)left;
	hwllc r = *(hwllc*)right;

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
	if (l.c < r.c) {
		return -1;
	}
	if (l.c > r.c) {
		return +1;
	}
	return 0;
}

bool isinrange (sll left, sll x, sll right) {
	return (left <= x && x <= right);
}

bool isinrange_soft (sll left, sll x, sll right) {
	return (left <= x && x <= right) || (left >= x && x >= right);
}

ull parent[N_MAX];
ull rank[N_MAX];
void uf_init (ull n) {
	for (sll i = 0; i < n; i++) {
		parent[i] = i;
		rank[i] = 0;
	}
}
ull uf_find (ull x) {
	if (parent[x] == x) return x;
	return parent[x] = uf_find(parent[x]);
}
sll uf_union (ull a, ull b) {
	a = uf_find(a);
	b = uf_find(b);
	if (a == b) return -1;

	if (rank[a] > rank[b]) {
		a ^= (b ^= (a ^= b));
	}

	parent[a] = b;
	if (rank[a] == rank[b]) {
		rank[b]++;
	}

	return b;
}


sll a[N_MAX];
// sll a[3001][3001];
sll b[N_MAX];
sll c[N_MAX];
char s[N_MAX + 1];
// char s[3010][3010];
char t[N_MAX + 1];
// char t[3010][3010];
// hwll xy[N_MAX];

hwllc sorted[N_MAX];
ull slen[N_MAX];


sll dep[N_MAX];
sll col[N_MAX];

sll sums[N_MAX][2];

ull sumb (ull b, ull e, ull w) {
	return sums[b][w] - sums[e][w];
}

hwll dfs (ull v, ull depth, sll currc) {
	hwll range = {1, BIG};
	dep[v] = depth;
	col[v] = currc;


	for (sll i = slen[v]; i < slen[v + 1]; i++) {
		ull u = sorted[i].b;

		sums[depth + 1][0] = sums[depth][0];
		sums[depth + 1][1] = sums[depth][1];
		// sums[depth + 1][currc ^ ((col[u] == currc) ? 1 : 0)] += sorted[i].c;
		sums[depth + 1][currc] += sorted[i].c;

		// for (sll j = 0; j <= depth + 1; j++) {
		// 	printf("[%lld]: %llu, %llu\n", j, sums[j][0], sums[j][1]);
		// }
		// puts("");

		if (col[u] != -1) {
			if (col[u] == currc) {
				// puts("Kisu");
				sll x = sumb(depth + 1, dep[u], currc) - sumb(depth + 1, dep[u], currc ^ 1);
				// printf("%lld - %lld -> %lld?\n", sumb(depth + 1, dep[u], currc), sumb(depth + 1, dep[u], currc ^ 1), x);
				if (x <= 0 || x % 2) {
					range.b = 0;
					return range;
				} else {
					range.a = -u;
					range.b = x / 2;
					return range;
				}
			} else {
				// if (dep[u] + 1 < depth) abort();
				// puts("Gusu");
				// if (sums[depth][0] - sums[dep[u]][0] != sums[depth][1] - sums[dep[u]][1]) {
				// printf("%llu - %llu == %llu - %llu ?\n", sums[depth + 1][0], sums[dep[u]][0], sums[depth + 1][1], sums[dep[u]][1]);
				if (sumb(depth + 1, dep[u], 0) != sumb(depth + 1, dep[u], 1)) {
					if (dep[u] + 1 == depth) abort();
					range.b = 0;
					return range;
				}
			}
		} else {
			// printf("Let's go %lld\n", u);
			hwll ret = dfs(u, depth + 1, currc ^ 1);
			if (ret.a <= 0 || ret.b == 0) return ret;

			hwll newr = {sorted[i].c - ret.b, sorted[i].c - ret.a};
			if (newr.a > range.a) range.a = newr.a;
			if (newr.b < range.b) range.b = newr.b;
			if (range.a <= 0 || range.a > range.b) {
				range.b = 0;
				return range;
			}
		}
	}

	// printf("Get out %lld\n", v);


	return range;
}

sll detx[N_MAX];
bool det (ull v, ull x) {
	detx[v] = x;
	// printf("%lld: %llu\n", v, x);

	for (sll i = slen[v]; i < slen[v + 1]; i++) {
		ull u = sorted[i].b;

		if (sorted[i].c <= x) return false;

		if (detx[u]) {
			if (detx[u] + x != sorted[i].c) return false;
		} else {
			sll newx = sorted[i].c - detx[v];
			if (!det(u, newx)) return false;
		}
	}

	return true;
}

ull solve () {
	sll i, j, ki, li;
	// ull result = 0;
	sll result = 0;
	double dresult = 0;
	// ull maybe = 0;
	sll maybe = 0;
	// ull sum = 0;
	sll sum = 0;
	sll item;
	ull *dpcell;

	for (i = 0; i < m; i++) {
		sorted[i].a = sorted[i + m].b = a[i];
		sorted[i].b = sorted[i + m].a = b[i];
		sorted[i].c = sorted[i + m].c = c[i];
	}
	qsort(sorted, m * 2, sizeof(hwllc), ptriplecomp);
	i = j = 0;
	while (i < n) {
		slen[i] = j;
		while (j < m * 2 && sorted[j].a == i) {
			j++;
		}

		i++;
	}
	slen[n] = m * 2;

	for (i = 0; i < n; i++) col[i] = -1;

	dep[0] = 0;
	col[0] = 0;
	sums[0][0] = sums[0][1] = 0;
	hwll ret = dfs(0, 0, 0);


	if (ret.b == 0) {
		result = 0;
	} else if (ret.a <= 0) {
		ull v = -ret.a;

		result = (det(v, ret.b) ? 1 : 0);
	} else {
		result = (ret.b - ret.a + 1);
	}



	printf("%llu\n", result);
	// printf("%.15lf\n", dresult);
	// puts(s);

	return 0;

	success:
	// puts("YES");
	// puts("Yes");
	// printf("%llu\n", result);
	// puts("0");
	// puts("First");
	return 0;

	fail:
	// puts("NO");
	// puts("No");
	// puts("0");
	puts("-1");
	// puts("-1 -1 -1");
	// puts("Second");
	return 1;
}

int32_t main (void) {
	int32_t i, j;
	int32_t x, y;

	// scanf("%lf%lf", &vda, &vdb, &vdc);
	// scanf("%lld%lld", &vsa, &vsb, &vsc, &vsd);
	// scanf("%llu%llu", &vua, &vub, &vuc, &vud);
	// scanf("%*llu%*llu");
	// scanf("%llu%llu", &h, &w);
	scanf("%llu%llu", &n, &m);
	// scanf("%llu", &k, &n, &m);
	// scanf("%llu%llu", &vua, &vub, &vuc, &vud, &vue, &vuf);
	// vua--;
	// vub--;
	// vuc--;
	// vud--;
	// scanf("%lld%lld", &vsa, &vsb, &vsc);
	// scanf("%s", s);
	// scanf("%s", t);
	// scanf("%llu", &k);
	// scanf("%llu", &q);
	for (i = 0; i < m; i++) {
		// scanf("%lld%lld", &xy[i].a, &xy[i].b);
		scanf("%lld", &a[i]);
		scanf("%lld", &b[i]);
		scanf("%lld", &c[i]);
		// scanf("%lld", &t[i]);
		a[i]--;
		b[i]--;
		// c[i]--;
	}
	// scanf("%llu%llu", &m, &k);
	// for (i = 0; i < n; i++) {
	// 	scanf("%llu", &b[i]);
	// 	// b[i]--;
	// }
	// scanf("%llu", &q);

	// for (i = 0; i < n; i++) {
	// 	for (j = 0; j < n; j++) {
	// 		scanf("%llu", &a[i][j]);
	// 	}
	// }
	// for (i = 0; i < h; i++) {
	// 	scanf("%s", &s[i]);
	// }

	// scanf("%llu", &k);
	// scanf("%llu", &q);

	solve();

	return 0;
}
