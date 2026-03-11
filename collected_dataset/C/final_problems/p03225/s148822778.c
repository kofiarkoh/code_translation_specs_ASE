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

int32_t prevcomp (const void *left, const void *right) {
	ull l = *(ull*)left;
	ull r = *(ull*)right;
	if (l > r) {
		return -1;
	}
	if (l < r) {
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

int32_t pdoublecomp (const void *left, const void *right) {
	double l = *(double*)left;
	double r = *(double*)right;
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

int32_t ptripleREVcomp (const void *left, const void *right) {
	hwllc l = *(hwllc*)left;
	hwllc r = *(hwllc*)right;

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
	if (l.c < r.c) {
		return -1;
	}
	if (l.c > r.c) {
		return +1;
	}
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

sll a[N_MAX];
// ull a[N_MAX];
// sll a[3001][3001];
sll b[N_MAX];
// sll b[3001][3001];
sll c[N_MAX];
sll d[N_MAX];
// sll e[N_MAX];
// char s[N_MAX + 1];
char s[3010][3010];
char t[N_MAX + 1];
// char t[3010][3010];
hwll xy[N_MAX];
hwllc tup[N_MAX];
sll table[3000][3000];
// here we go

ull dp[305][305][602], dq[305][305][602];

sll fp (sll i, sll j, sll x) {
	if (j >= (sll)w) {
		sll d = j - (w - 1);
		i -= d;
		j -= d;
	}
	if (i >= (sll)h) {
		sll d = i - (h - 1);
		i -= d;
		j -= d;
	}
	if (i < 0 || j < 0) return 0;
	return dp[i][j][x];
}
sll fq (sll i, sll j, sll x) {
	// printf("%lld %lld->\n", i, j);
	if (j < 0) {
		sll d = -j;
		i -= d;
		j += d;
	}
	if (i >= (sll)h) {
		sll d = i - (h - 1);
		i -= d;
		j += d;
	}
	// printf("%lld %lld\n", i, j);
	if (i < 0 || j >= (sll)w) return 0;
	return dq[i][j][x];
}

ull f (sll i, sll j) {
	sll mov[4][2][2] = {
		{{-1, -1}, {-1, +1}},
		{{-1, +1}, {+1, +1}},
		{{+1, +1}, {+1, -1}},
		{{+1, -1}, {-1, -1}}
		// ,
		// {{-3, -1}, {-1, -3}},
		// {{-3, +1}, {-1, +3}},
		// {{+3, -1}, {+1, -3}},
		// {{+3, +1}, {+1, +3}}
	};

	ull ra = 0;
	for (sll ki = 0; ki < 4; ki++) {
		sll x[2][2] = {
			{i, j}, {i, j}
		};

		while (true) {

			for (sll xi = 0; xi < 2; xi++) for (sll xj = 0; xj < 2; xj++) {
				x[xi][xj] += mov[ki][xi][xj];
			}
			if (!(
			isinrange(0, x[0][0], h - 1)
			&& isinrange(0, x[0][1], w - 1)
			&& isinrange(0, x[1][0], h - 1)
			&& isinrange(0, x[1][1], w - 1))) break;

			if (s[x[0][0]][x[0][1]] == '#' && s[x[1][0]][x[1][1]] == '#') {
				// printf("%lld:%lld: %lld:%lld,%lld:%lld (%lld)\n", i, j, x[0][0], x[0][1], x[1][0], x[1][1], ki);
				ra++;
			}
		}
	}

	sll r = 0;
	sll ki;
	sll maxd = smax(sdiff(i, 0), sdiff(i, h - 1)) + smax(sdiff(j, 0), sdiff(j, w - 1));

	maxd = sdiff(i, 0) + sdiff(j, 0);
	for (ki = 1; ki <= maxd; ki++) {
		r += fq(i, j - ki * 2, ki);
		// printf("%lld...\n", r);
		r -= fq(i - ki - 1, j - ki + 1, ki);
		// printf("%lld...\n", r);
	}
	maxd = sdiff(i, 0) + sdiff(j, w - 1);
	for (ki = 1; ki <= maxd; ki++) {
		r += fp(i, j + ki * 2, ki);
		// printf("%lld...\n", r);
		r -= fp(i - ki - 1, j + ki - 1, ki);
		// printf("%lld...\n", r);
	}
	maxd = sdiff(i, h - 1) + sdiff(j, h - 1);
	for (ki = 1; ki <= maxd; ki++) {
		r += fp(i + ki * 2, j, ki);
		// printf("%lld...\n", r);
		r -= fp(i + ki - 1, j - ki - 1, ki);
		// printf("%lld...\n", r);
	}
	maxd = sdiff(i, h - 1) + sdiff(j, w - 1);
	for (ki = 1; ki <= maxd; ki++) {
		r += fq(i + ki * 2, j, ki);
		// printf("%lld...\n", r);
		r -= fq(i + ki - 1, j + ki + 1, ki);
		// printf("%lld...\n", r);
	}
	// for (ki = 1; ki <= maxd; ki++) 


	// 	// printf("%lld-%lld: %lld(%lld\n", i, j, r, ki);
	// }

	// printf("\t%lld-%lld: %llu\n", i, j, r);
	return r - ra;
}

sll ok (sll x1, sll y1, sll x2, sll y2, sll x3, sll y3) {
	sll z1 = sdiff(x1, x2) + sdiff(y1, y2);
	sll z2 = sdiff(x2, x3) + sdiff(y2, y3);
	sll z3 = sdiff(x3, x1) + sdiff(y3, y1);

	if (z1 == z2 && z2 == z3) {
		return 1;
	}
	return 0;
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

	// sll x1, y1, x2, y2, x3, y3;
	// for (x1 = 0; x1 < h; x1++) {
	// 	for (x2 = 0; x2 < h; x2++) {
	// 		for (x3 = 0; x3 < h; x3++) {
	// 			for (y1 = 0; y1 < w; y1++) {
	// 				for (y2 = 0; y2 < w; y2++) {
	// 					for (y3 = 0; y3 < w; y3++) {
	// 						if (s[x1][y1] == '#' && s[x2][y2]== '#' && s[x3][y3] == '#') {
	// 							if (ok(x1, y1, x2, y2, x3, y3)) {
	// 								printf("%lld %lld %lld %lld %lld %lld\n", x1, y1, x2, y2, x3, y3);
	// 							}
	// 						}
	// 					}
	// 				}
	// 			}
	// 		}
	// 	}
	// }

	for (ki = 1; ki <= h; ki++) {
		// printf("\t%lld\n", ki);
		for (i = 0; i < h; i++) {
			for (j = 0; j < w; j++) {
				if (i < ki) {
					dp[i][j][ki] = 0;
					dq[i][j][ki] = 0;
					continue;
				}

				if (j - ki < 0) {
					dp[i][j][ki] = 0;
				} else {
					dp[i][j][ki] = dp[i - 1][j - 1][ki];
					if (s[i][j] == '#' && s[i - ki][j - ki] == '#') {
						dp[i][j][ki]++;
					}
				}
				if (j + ki >= (sll)w) {
					dq[i][j][ki] = 0;
				} else {
					dq[i][j][ki] = dq[i - 1][j + 1][ki];
					if (s[i][j] == '#' && s[i - ki][j + ki] == '#') {
						dq[i][j][ki]++;
					}
				}
				// printf("%lld-%lld ", dp[i][j][ki], dq[i][j][ki]);
			}
			// puts("");
		}
	}

	for (i = 0; i < h; i++) {
		for (j = 0; j < w; j++) {
			if (s[i][j] == '#') result += f(i, j);
		}
	}

	// fp(0, -1, 1);
	// printf("%lld\n", fp(0, -1, 1));

	printf("%lld\n", result);
	// printf("%.15lf\n", dresult);
	// puts(s);

	return 0;

	success:
	// puts("YES");
	puts("Yes");
	// printf("%llu\n", result);
	// puts("0");
	// puts("Yay!");
	return 0;

	fail:
	// puts("NO");
	puts("No");
	// puts("0");
	// puts("-1");
	// puts("-1 -1 -1");
	// puts("Impossible");
	return 1;
}

int32_t main (void) {
	int32_t i, j;
	int32_t x, y;



	scanf("%llu%llu", &h, &w);
	// scanf("%llu", &n, &m);
	// scanf("%llu", &k, &n, &m);
	// scanf("%llu%llu", &h, &w);
	// scanf("%llu", &q);
	// scanf("%s", s);
	// scanf("%lld%lld", &va, &vb, &vc, &vd);
	// scanf("%llu%llu", &ua, &ub, &uc, &ud);
	// scanf("%s", t);
	// for (i = 0; i < n; i++) {
	// 	// scanf("%lld%lld", &xy[i].a, &xy[i].b);
	// 	// scanf("%lld%lld%lld", &tup[i].a, &tup[i].b, &tup[i].c);
	// 	scanf("%lld", &a[i]);
	// 	// scanf("%lld", &b[i]);
	// 	// scanf("%lld", &c[i]);
	// 	// scanf("%lld", &d[i]);
	// 	// a[i]--;
	// 	// b[i]--;
	// 	// c[i]--;
	// 	// d[i]--;
	// 	// tup[i].a--;
	// 	// tup[i].b--;
	// }
	// scanf("%llu", &m, &k);
	// scanf("%llu", &q);
	// scanf("%s", s);
	// for (i = 0; i < m; i++) {
	// 	scanf("%lld", &b[i]);
	// 	// b[i]--;
	// }

	// for (i = 0; i < h; i++) {
	// 	for (j = 0; j < w; j++) {
	// 		scanf("%llu", &table[i][j]);
	// 	}
	// }
	for (i = 0; i < h; i++) {
		scanf("%s", &s[i]);
	}

	solve();

	return 0;
}
