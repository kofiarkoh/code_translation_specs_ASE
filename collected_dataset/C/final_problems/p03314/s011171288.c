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

int bitcount (ull n) {
	int result = 0;
	while (n) {
		if (n & 1) result++;
		n /= 2;
	}
	return result;
}

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
// sll b[3001][3001];
sll c[N_MAX];
char s[N_MAX + 1];
// char s[3010][3010];
// char t[N_MAX + 1];
// char t[3010][3010];
// hwll xy[N_MAX];

ull dp1[30000][500], dp2[30000][500];
ull ac1[30000][500], ac2[30000][500];

ull checker[30000];

ull colorful () {
	sll i, j;

	dp1[0][0] = 1;
	ac1[0][0] = 1;

	for (i = 0; i <= n; i++) {
		for (j = 1; j < k; j++) {
			if (i) {
				dp1[i][j] = MOD + ac1[i - 1][k - 1] - ac1[i - 1][j - 1] + (dp1[i - 1][j - 1] * (k - j + 1) % MOD);
				dp1[i][j] %= MOD;
				dp2[i][j] = MOD + ac2[i - 1][k - 1] - ac2[i - 1][j - 1] + (dp2[i - 1][j - 1] * (k - j + 1) % MOD);
				dp2[i][j] %= MOD;

				if (j >= m) {
					dp2[i][j] = (dp1[i][j] + dp2[i][j]) % MOD;
				}
			}

			ac1[i][j] = (ac1[i][j - 1] + dp1[i][j]) % MOD;
			ac2[i][j] = (ac2[i][j - 1] + dp2[i][j]) % MOD;
		}
	}

	ull result = ac2[n][k - 1];
	for (i = 0; i < m; i++) {
		result = divide(result, k - i, MOD);
	}

	return result;
}

ull blocked (ull bef, ull aft) {
	sll i, j;

	dp1[0][bef] = 1;
	dp2[0][aft] = 1;

	for (i = 0; i <= n; i++) {
		for (j = 1; j < k; j++) {
			if (i) {
				dp1[i][j] = MOD + ac1[i - 1][k - 1] - ac1[i - 1][j - 1] + (dp1[i - 1][j - 1] * (k - j + 1) % MOD);
				dp1[i][j] %= MOD;
				dp2[i][j] = MOD + ac2[i - 1][k - 1] - ac2[i - 1][j - 1] + (dp2[i - 1][j - 1] * (k - j + 1) % MOD);
				dp2[i][j] %= MOD;
			}

			ac1[i][j] = (ac1[i][j - 1] + dp1[i][j]) % MOD;
			ac2[i][j] = (ac2[i][j - 1] + dp2[i][j]) % MOD;
		}
	}

	ull result = 0;
	for (i = 0; i + m <= n; i++) {
		result += (ac1[i][k - 1] * ac2[n - m - i][k - 1] % MOD);
		result %= MOD;
	}
	return result;
}

bool acol () {
	sll i;
	for (i = 0; i < k; i++) checker[i] = ~0;
	ull nearest = 0;
	for (i = 0; i < m; i++) {
		if (~checker[a[i]] && nearest <= checker[a[i]]) {
			nearest = checker[a[i]] + 1;
		}
		checker[a[i]] = i;

		if (i - nearest + 1 >= k) return true;
	}

	return false;
}

ull solve () {
	sll i, j, ki, li;
	ull result = 0;
	// sll result = 0;
	double dresult = 0;
	// ull maybe = 0;
	sll maybe = 0;
	// ull sum = 0;
	sll sum = 0;
	sll item;
	ull *dpcell;

	ull bef, aft;

	for (i = 0; i < m; i++) {
		if (checker[a[i]] == 1) break;
		checker[a[i]] = 1;
	}
	bef = i;
	for (i = 0; i < m; i++) {
		if (checker[a[m - 1 - i]] == 2) break;
		checker[a[m - 1 - i]] = 2;
	}
	aft = i;

	if (acol()) {
		result = 0;
	} else if (bef == m) {
		result = colorful();
	} else {
		result = blocked(bef, aft);
	}

	result = (MOD + ((n - m + 1) * bitpow(k, n - m, MOD) % MOD) - result) % MOD;

	printf("%llu\n", result);
	// printf("%.15lf\n", dresult);
	// puts(s);

	return 0;

	success:
	// puts("YES");
	// puts("Yes");
	// printf("%llu\n", result);
	// puts("0");
	// puts("Takahashi");
	return 0;

	fail:
	// puts("NO");
	// puts("No");
	puts("0");
	// puts("-1");
	// puts("-1 -1 -1");
	// puts("Aoki");
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
	scanf("%llu", &n, &m);
	scanf("%llu", &k, &n, &m);
	scanf("%llu", &m);
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
		// scanf("%lld", &b[i]);
		// scanf("%lld", &c[i]);
		// scanf("%lld", &t[i]);
		a[i]--;
		// b[i]--;
		// c[i]--;
	}
	// scanf("%llu%llu", &m, &k);
	// for (i = 0; i < n; i++) {
	// 	scanf("%llu", &b[i]);
	// 	// b[i]--;
	// }
	// scanf("%llu", &q);

	// for (i = 0; i < h; i++) {
	// 	for (j = 0; j < w; j++) {
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
