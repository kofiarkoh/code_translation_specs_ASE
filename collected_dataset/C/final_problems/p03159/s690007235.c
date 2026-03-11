#if 0
cat <<EOF >mistaken-paste
#endif
// thanks for @rsk0315_h4x

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

#ifdef __cplusplus
#include <queue>
#include <stack>
#include <tuple>
#include <set>
#include <map>
#include <string>
#include <algorithm>
#include <functional>
#include <array>

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
using std::get;

template <typename T, typename U>
pair<T, U> operator+(pair<T, U> l, pair<T, U> r) {
	return pair<T, U>(l.first + r.first, l.second + r.second);
}

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
sll va, vb, vc, vd, ve, vf;
ull ua, ub, uc, ud, ue, uf;
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

int bitcount (ull n) {
	int result = 0;
	while (n) {
		if (n & 1) result++;
		n /= 2;
	}
	return result;
}

#define BEGCMP(NAME) int32_t NAME (const void *left, const void *right)
#define DEFLR(TYPE) TYPE l=*(TYPE*)left,r=*(TYPE*)right
#define CMPRET(L, R) if((L)<(R))return-1;if((L)>(R))return+1

// int32_t pullcomp (const void *left, const void *right) {
// 	ull l = *(ull*)left;
// 	ull r = *(ull*)right;
// 	if (l < r) {
// 		return -1;
// 	}
// 	if (l > r) {
// 		return +1;
// 	}
// 	return 0;
// }
BEGCMP(pullcomp){
	DEFLR(ull);
	CMPRET(l, r);
	return 0;
}
BEGCMP(prevcomp){
	DEFLR(ull);
	CMPRET(r, l);
	return 0;
}
BEGCMP(psllcomp){
	DEFLR(sll);
	CMPRET(l, r);
	return 0;
}
BEGCMP(pcharcomp){
	DEFLR(char);
	CMPRET(l, r);
	return 0;
}
BEGCMP(pdoublecomp){
	DEFLR(double);
	CMPRET(l, r);
	return 0;
}

int32_t pstrcomp (const void *left, const void *right) {
	char* l = *(char**)left;
	char* r = *(char**)right;

	return strcmp(l, r);
}

BEGCMP(phwllABcomp){
	DEFLR(hwll);
	CMPRET(l.a, r.a);
	CMPRET(l.b, r.b);
	return 0;
}
BEGCMP(phwllREVcomp){
	DEFLR(hwll);
	CMPRET(l.b, r.b);
	CMPRET(l.a, r.a);
	return 0;
}
BEGCMP(ptriplecomp){
	DEFLR(hwllc);
	CMPRET(l.a, r.a);
	CMPRET(l.b, r.b);
	CMPRET(l.c, r.c);
	return 0;
}
BEGCMP(ptripleREVcomp){
	DEFLR(hwllc);
	CMPRET(l.b, r.b);
	CMPRET(l.a, r.a);
	CMPRET(l.c, r.c);
	return 0;
}

int32_t pquadcomp (const void *left, const void *right) {
	linell l = *(linell*)left;
	linell r = *(linell*)right;

	sll ac = phwllABcomp(&(l.a), &(r.a));
	if (ac) return ac;
	sll bc = phwllABcomp(&(l.b), &(r.b));
	if (bc) return bc;

	return 0;
}

bool isinrange (sll left, sll x, sll right) {
	return (left <= x && x <= right);
}

bool isinrange_soft (sll left, sll x, sll right) {
	return (left <= x && x <= right) || (left >= x && x >= right);
}

sll a[N_MAX + 5];
// ull a[N_MAX + 5];
// sll a[3001][3001];
sll b[N_MAX + 5];
// ull b[N_MAX + 5];
// sll b[3001][3001];
sll c[N_MAX + 5];
ull d[N_MAX + 5];
sll e[N_MAX];
char s[N_MAX + 1];
// char s[3010][3010];
char t[N_MAX + 1];
// char t[3010][3010];
char u[N_MAX + 1];
hwll xy[N_MAX + 5];
hwllc tup[N_MAX + 5];
sll table[3005][3005];
// here we go

ull gin[N_MAX];
ull vvv (ull v, ull p) {
	e[v] = 1;
	for (sll i = gin[v]; i < gin[v + 1]; i++) {
		ull u = xy[i].b;
		if (u == p) continue;
		d[u] = d[v] + 1;
		e[v] += vvv(u, v);
	}
	return e[v];
}

sll dp[5005][2][5005];
ull dfs (ull v, ull p) {
	sll tmp[2][e[v]];

	for (sll i = 0; i <= n; i++) {
		dp[v][0][i] = dp[v][1][i] = VERYBIG;
	}
	dp[v][(c[v] < 0) ? 1 : 0][0] = c[v];
	// printf("%lld %lld: %lld\n", v, 0LL, c[v]);
	ull maxv = 1;
	for (sll i = gin[v]; i < gin[v + 1]; i++) {
		ull u = xy[i].b;
		if (u == p) continue;
		ull maxu = dfs(u, v);

		for (sll x = 0; x < e[v]; x++) tmp[0][x] = tmp[1][x] = VERYBIG;
		for (sll x = 0; x <= maxv - 1; x++) {
			for (sll y = 0; y <= maxu - 1; y++) {

				for (sll z1 = 0; z1 <= 1; z1++) {

					for (sll z2 = 0; z2 <= 1; z2++) {

						tmp[z1 | z2][x + y] = smin(tmp[z1 | z2][x + y], dp[v][z1][x] + dp[u][z2][y]);
						// printf("%lld+%lld [%lld] <- %lld\n", x, y, (z1 | z2), tmp[z1 | z2][x + y]);
						if ((!z2 && dp[u][z2][y] != VERYBIG) || dp[u][z2][y] < 0) {
							tmp[z1][x + y + 1] = smin(tmp[z1][x + y + 1], dp[v][z1][x]);
							// printf("%lld+%lld+1 [%lld] <- %lld (%lld %lld %lld: %lld)\n", x, y, z1, tmp[z1][x + y + 1], u, z2, y, dp[u][z2][y]);
						}
					}
				}

			}
		}
		for (sll x = 0; x < e[v]; x++) {
			dp[v][0][x] = tmp[0][x];
			dp[v][1][x] = tmp[1][x];
			// printf("%lld/%lld: %lld, %lld\n", v, x, dp[v][0][x], dp[v][1][x]);
		}
		maxv += maxu;
	}
	// printf("%lld: %llu %s\n", v, maxv, (hascon[v] ? "HAS" : "NO"));
	return maxv;
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

	for (i = 0; i < n - 1; i++) {
		xy[i * 2] = (hwll){a[i], b[i]};
		xy[i * 2 + 1] = (hwll){b[i], a[i]};
	}
	qsort(xy, (n - 1) * 2, sizeof(hwll), phwllABcomp);
	i = j = 0;
	while (i <= n) {
		gin[i] = j;
		while (j < (n - 1) * 2 && xy[j].a == i) j++;
		i++;
	}

	ull v = 0;
	vvv(v, n);
	for (i = 0; i < n; i++) if (d[i] > d[v]) v = i;
	vvv(v, n);
	ull u = 0;
	for (i = 0; i < n; i++) if (d[i] > d[u]) u = i;
	for (i = 0; i < n; i++) if (d[i] == d[u] / 2) break;
	v = i;
	vvv(v, n);

	dfs(v, n);

	for (i = 0; i <= n; i++) {
		if (dp[v][0][i] != VERYBIG || dp[v][1][i] < 0) break;
	}
	result = i;

	printf("%lld\n", result);
	// printf("%.15lf\n", dresult);
	// puts(s);

	return 0;

	success:
	puts("YES");
	// puts("Yes");
	// printf("%llu\n", result);
	// puts("0");
	// puts("Yay!");
	return 0;

	fail:
	// puts("NO");
	// puts("No");
	// puts("0");
	puts("-1");
	// puts("-1 -1 -1");
	// puts("Impossible");
	return 1;
}

int32_t main (void) {
	int32_t i, j;
	int32_t x, y;



	// scanf("%llu%llu", &h, &w);
	scanf("%llu", &n, &m);
	// scanf("%llu", &k, &n, &m);
	// scanf("%llu%llu", &h, &w);
	// scanf("%llu", &q);
	// scanf("%s", s);
	// scanf("%lld%lld", &va, &vb, &vc, &vd);
	// scanf("%llu%llu%llu", &ua, &ub, &uc, &ud);
	// scanf("%s", t);
	// scanf("%lld", &m);
	for (i = 0; i < n; i++) {
		// scanf("%lld%lld", &xy[i].a, &xy[i].b);
		// scanf("%lld%lld%lld", &tup[i].a, &tup[i].b, &tup[i].c);
		// scanf("%lld", &a[i]);
		// scanf("%lld", &b[i]);
		scanf("%lld", &c[i]);
		// scanf("%lld", &d[i]);
		// a[i]--;
		// b[i]--;
		// c[i]--;
		// d[i]--;
		// xy[i].a--;
		// xy[i].b--;
		// tup[i].a--;
		// tup[i].b--;
	}
	for (i = 0; i < n - 1; i++) {
		scanf("%lld%lld", &a[i], &b[i]);
		a[i]--;
		b[i]--;
	}
	// scanf("%llu", &m, &k);
	// scanf("%llu", &q);
	// scanf("%s", s);
	// for (i = 0; i < n; i++) {
	// 	scanf("%lld", &a[i]);
	// 	// scanf("%lld", &b[i]);
	// 	// b[i]--;
	// 	// c[i]--;
	// }
	// scanf("%llu", &q);

	// for (i = 0; i < q; i++) {
	// 	scanf("%lld%lld", &xy[i].a, &xy[i].b);
	// 	xy[i].a--;
	// 	xy[i].b--;
	// }

	// for (i = 0; i < n; i++) {
	// 	for (j = 0; j < n; j++) {
	// 		scanf("%lld", &table[i][j]);
	// 		// table[i][j]--;
	// 	}
	// }
	// for (i = 0; i < h; i++) {
	// 	scanf("%s", s[i]);
	// }
	// scanf("%llu", &q);

	solve();

	return 0;
}
