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
// ull a[N_MAX];
// sll a[3001][3001];
sll b[N_MAX + 5];
// sll b[3001][3001];
sll c[N_MAX + 5];
sll d[N_MAX + 5];
// sll e[N_MAX];
char s[N_MAX + 1];
// char s[3010][3010];
char t[N_MAX + 1];
// char t[3010][3010];
hwll xy[N_MAX];
hwllc tup[N_MAX];
sll table[3000][3000];
// here we go

ull gin[N_MAX];

const ull inv2 = 500000004LL;
ull dp[2000][2000]/*[2]*/[5];

ull dfs (ull v) {
	ull tmp[2000]/*[2]*/[5];
	dp[v][0]/*[0]*/[0] = 1;

	ull left = 1;
	for (ull i = gin[v]; i < gin[v + 1]; i++) {
		ull u = xy[i].b;
		ull right = dfs(u);

		// printf("%llu->%llu (%lld += %lld)..\n", v, u, left, right);
		fflush(stdout);

		for (sll j = 0; j < left + right; j++) {
			// for (sll k = 0; k < 2; k++) {
				for (sll l = 0; l < 5; l++) {
					tmp[j]/*[k]*/[l] = 0;
				}
			// }
		}

		for (sll jl = 0; jl < left; jl++) {
			for (sll jr = 0; jr < right; jr++) {
				sll j = jl + jr;
				// for (ull kl = 0; kl < 2; kl++) {
				// 	for (ull kr = 0; kr < 2; kr++) {
				// 		ull k = kl ^ kr;

						ull l0 = dp[v][jl]/*[kl]*/[0];
						ull l1 = dp[v][jl]/*[kl]*/[1];
						ull l2 = dp[v][jl]/*[kl]*/[2];
						ull l3 = dp[v][jl]/*[kl]*/[3];
						ull l4 = dp[v][jl]/*[kl]*/[4];
						ull r0 = dp[u][jr]/*[kr]*/[0];
						ull r1 = dp[u][jr]/*[kr]*/[1];
						ull r2 = dp[u][jr]/*[kr]*/[2];

						tmp[j + 1]/*[k]*/[0] += l0 * (r0 + r1 + r2) % MOD;
						tmp[j + 1]/*[k]*/[0] %= MOD;
						tmp[j]/*[k ^ 1]*/[1] += (l0 * r0 % MOD) * 2;
						tmp[j]/*[k ^ 1]*/[1] %= MOD;
						tmp[j]/*[k]*/[1] += l0 * (r1) % MOD;
						tmp[j]/*[k]*/[1] %= MOD;
						tmp[j]/*[k]*/[3] += l0 * r0 % MOD;
						tmp[j]/*[k]*/[3] %= MOD;
						tmp[j]/*[k]*/[4] += l0 * (r1) % MOD;
						tmp[j]/*[k]*/[4] %= MOD;

						tmp[j + 1]/*[k]*/[1] += l1 * (r0 + r1 + r2) % MOD;
						tmp[j + 1]/*[k]*/[1] %= MOD;

						tmp[j + 1]/*[k]*/[2] += l2 * (r0 + r1 + r2) % MOD;
						tmp[j + 1]/*[k]*/[2] %= MOD;

						tmp[j + 1]/*[k]*/[3] += l3 * (r0 + r1 + r2) % MOD;
						tmp[j + 1]/*[k]*/[3] %= MOD;
						tmp[j]/*[k ^ 1]*/[2] += (l3 * r0 % MOD) * 2;
						tmp[j]/*[k ^ 1]*/[2] %= MOD;
						tmp[j]/*[k]*/[2] += l3 * r1 % MOD;
						tmp[j]/*[k]*/[2] %= MOD;

						tmp[j + 1]/*[k]*/[4] += l4 * (r0 + r1 + r2) % MOD;
						tmp[j + 1]/*[k]*/[4] %= MOD;
						tmp[j]/*[k]*/[2] += l4 * r0 % MOD;
						tmp[j]/*[k]*/[2] %= MOD;
						tmp[j]/*[k ^ 1]*/[2] += (l4 * r1 % MOD) * inv2 % MOD;
						tmp[j]/*[k ^ 1]*/[2] %= MOD;


				// 	}
				// }
			}
		}

		left += right;
		for (sll j = 0; j < left; j++) {
			// for (sll k = 0; k < 2; k++) {
				for (sll l = 0; l < 5; l++) {
					dp[v][j]/*[k]*/[l] = tmp[j]/*[k]*/[l];
					// printf("%lld:%lld:%lld:%lld : %lld\n", v, j, k, l, dp[v][j][k][l]);
				}
			// }
		}
	}

	// printf("%lld done\n", v);
	fflush(stdout);

	return left;
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

	for (i = 1; i < n; i++) {
		xy[i - 1] = (hwll){a[i], i};
	}
	qsort(xy, n - 1, sizeof(hwll), phwllABcomp);
	i = j = 0;
	while (i < n) {
		gin[i] = j;
		while (j < n - 1 && xy[j].a == i) j++;
		i++;
	}
	gin[i] = j;

	dfs(0);

	ull frac = 1;
	for (j = 0; j < n; j++) {
		frac *= (j + 1);
		frac %= MOD;
		ull item = 0;
		// for (li = 0; li < 3; li++) {
		// 	item = (item + (dp[0][j][0][li] + MOD - dp[0][j][1][li])) % MOD;
		// }
		// for (ki = 0; ki < 2; ki++) {
			for (li = 0; li < 3; li++) {
				item = (item + dp[0][j]/*[ki]*/[li]) % MOD;
			}
		// }
		// printf("%llu : %llu\n", j, item);
		item = (item * frac) % MOD;
		// result = (result + (item * frac % MOD)) % MOD;
		if ((n - 1 - j) % 2) {
			result = (result + MOD - item) % MOD;
		} else {
			result = (result + item) % MOD;
		}
	}

	printf("%lld\n", result);
	// printf("%.15lf\n", dresult);
	// puts(s);

	return 0;

	success:
	// puts("YES");
	// puts("Yes");
	printf("%llu\n", result);
	// puts("0");
	// puts("Yay!");
	return 0;

	fail:
	// puts("NO");
	// puts("No");
	puts("0");
	// puts("-1");
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
	// scanf("%llu%llu", &ua, &ub, &uc, &ud);
	// scanf("%s", t);
	for (i = 1; i < n; i++) {
		// scanf("%lld%lld", &xy[i].a, &xy[i].b);
		// scanf("%lld%lld%lld", &tup[i].a, &tup[i].b, &tup[i].c);
		scanf("%lld", &a[i]);
		// scanf("%lld", &b[i]);
		// scanf("%lld", &c[i]);
		// scanf("%lld", &d[i]);
		a[i]--;
		// b[i]--;
		// c[i]--;
		// d[i]--;
		// tup[i].a--;
		// tup[i].b--;
	}
	// scanf("%llu", &m, &k);
	// scanf("%llu", &q);
	// scanf("%s", s);
	// for (i = 0; i < n; i++) {
	// 	scanf("%lld", &b[i]);
	// 	// b[i]--;
	// }
	// scanf("%llu", &q);

	// for (i = 0; i < h; i++) {
	// 	for (j = 0; j < w; j++) {
	// 		scanf("%llu", &table[i][j]);
	// 	}
	// }
	// for (i = 0; i < h; i++) {
	// 	scanf("%s", &s[i]);
	// }

	solve();

	return 0;
}
