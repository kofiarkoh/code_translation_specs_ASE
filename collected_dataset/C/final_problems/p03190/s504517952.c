#if 0
cat <<EOF >mistaken-paste
#endif
// thx Ebi-chan!

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

#define N_MAX 1048576

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

typedef struct {
	double a;
	double b;
} hwreal;

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
	DEFLR(sll);
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
BEGCMP(phwrealcomp){
	DEFLR(hwreal);
	CMPRET(l.a, r.a);
	CMPRET(l.b, r.b);
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
sll d[N_MAX + 5];
sll e[N_MAX * 4];
char s[N_MAX + 1];
// char s[3010][3010];
char t[N_MAX + 1];
// char t[3010][3010];
char u[N_MAX + 1];
hwll xy[N_MAX + 5];
hwllc tup[N_MAX + 5];
// sll table[3005][3005];
ull gin[N_MAX];
// here we go

sll la, lb;

bool isok[N_MAX];

sll so (sll a[], sll n) {
	if (n == 1) return 0;
	sll l = n / 2, r = n - l;
	sll inv = so(a, l) + so(a + l, r);

	sll i = 0, j = 0;

	sll tmp[n];
	while (i < l || j < r) {
		bool isr;
		if (i == l) {
			isr = true;
		} else if (j == r) {
			isr = false;
		} else if (a[i] > a[l + j]) {
			isr = true;
		} else {
			isr = false;
		}

		if (isr) {
			inv += (l - i);
			tmp[i + j] = a[l + j];
			j++;
		} else {
			tmp[i + j] = a[i];
			i++;
		}
	}
	for (i = 0; i < n; i++) {
		a[i] = tmp[i];
	}
	return inv;
}

sll func (sll a[], sll b[]) {

	sll i, j;
	sll x = 0, y = lb - 1;
	for (i = 0; i < lb; i++) {
		if (a[i] == 1) {
			e[x++] = i;
		} else {
			e[y--] = i;
		}
	}
	x = 0;
	y = lb - 1;
	for (i = 0; i < lb; i++) {
		if (b[i] == 1) {
			d[i] = e[x++];
		} else {
			d[i] = e[y--];
		}
	}

	return so(d, lb);
}

sll f (sll l, sll r) {
	sll i, j;
	// printf("%lld - %lld\n", l, r);

	sll l11 = 0, l2 = 0, r11 = 0, r2 = 0;
	sll c1 = 0, c2 = 0;

	for (i = 0; i < la; i++) {
		if (a[i] == 1) c1++;
		else c2++;
	}

	sll x = 0;

	for (i = 0; i < la; i++) {
		isok[i] = true;
	}

	if (l > 0) {
		j = 0;
		for (l11 = 0; l11 < la && j < l * 2; l11++) {
			if (a[l11] == 1) {
				x += (l11 - j);
				j++;
				isok[l11] = false;
				if (j == l * 2) break;
			}
		}
		if (l11 == la) return VERYBIG;

		x += l * l;
		// printf("l11: %lld\n", );
	} else {
		j = 0;
		for (l2 = 0; l2 < la && j < -l; l2++) {
			if (a[l2] == 2) {
				x += (l2 - j);
				j++;
				isok[l2] = false;
				if (j == -l) break;
			}
		}
		// printf("l2: %lld j: %lld\n", l2, j);
		if (l2 == la) return VERYBIG;

		x += l * l;
	}

	if (r > 0) {
		j = 0;
		for (r11 = la - 1; r11 >= 0 && j < r * 2; r11--) {
			if (a[r11] == 1) {
				x += ((la - 1 - j) - r11);
				j++;
				isok[r11] = false;
				if (j == r * 2) break;
			}
		}
		if (r11 < 0) return VERYBIG;

		x += r * r;
	} else {
		j = 0;
		for (r2 = la - 1; r2 >= 0 && j < -r; r2--) {
			if (a[r2] == 2) {
				x += ((la - 1 - j) - r2);
				j++;
				isok[r2] = false;
				if (j == -r) break;
			}
		}
		if (r2 < 0) return VERYBIG;

		x += r * r;
	}


	if (l > 0 && r > 0) {
		if (l11 >= r11) return VERYBIG;
	}
	if (l < 0 && r < 0) {
		if (l2 >= r2) return VERYBIG;
	}

	sll len = 0;
	if (l > 0) {
		for (i = 0; i < l; i++) {
			c[len++] = 2;
		}
	} else {
		for (i = 0; i < -l * 2; i++) {
			c[len++] = 1;
		}
	}

	for (i = 0; i < la; i++) {
		if (isok[i]) {
			c[len++] = a[i];
		}
	}

	if (r > 0) {
		for (i = 0; i < r; i++) {
			c[len++] = 2;
		}
	} else {
		for (i = 0; i < -r * 2; i++) {
			c[len++] = 1;
		}
	}

	// puts("c:");
	// for (i = 0; i < len; i++) {
	// 	printf("%lld ", c[i]);
	// }
	// puts("");

	// if (len != lb) {
	// 	fprintf(stdout, "%lld != %lld\n", len, lb);
	// }

	return x + func(c, b);
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
	sll *dpcell;

	la = 0;
	lb = 0;
	i = 0;
	while (i < n) {
		if (s[i] == s[i + 1]) {
			a[la++] = 2;
			i += 2;
		} else {
			a[la++] = 1;
			i += 1;
		}
	}
	i = 0;
	while (i < n) {
		if (t[i] == t[i + 1]) {
			b[lb++] = 2;
			i += 2;
		} else {
			b[lb++] = 1;
			i += 1;
		}
	}

	// for (i = 0; i < la; i++) {
	// 	printf("%lld\n", a[i]);
	// }

	sll incr2 = -(lb - la);

	result = VERYBIG;
	for (i = -(sll)n; i <= (sll)n; i++) {
		if ((i & 1LL) != ((s[0] != t[0]) ? 1 : 0)) {
			if (la == n || la * 2 == n || lb == n || lb * 2 == n) {
				result = smin(result, n + f(i, incr2 - i));
			}
			continue;
		}
		// printf("\t%lld\n", i);

		result = smin(result, f(i, incr2 - i));
		// printf("res : %lld\n", result);
	}

	printf("%lld\n", result);
	// printf("%.15lf\n", dresult);
	// puts(s);

	return 0;

	success:
	puts("YES");
	// puts("Yes");
	// printf("%llu\n", result);
	// puts("0");
	// puts("Hanako");
	return 0;

	fail:
	puts("NO");
	// puts("No");
	// puts("0");
	// puts("-1");
	// puts("-1 -1 -1");
	// puts("Jiro");
	return 1;
}

int32_t main (int argc, char *argv[]) {
	int32_t i, j;

	n = 3;
	m = 0;

	// scanf("%llu%llu", &h, &w);
	scanf("%llu", &n, &m);
	// scanf("%llu", &k, &n, &m);
	// scanf("%llu%llu", &h, &w);
	// scanf("%llu", &q);
	scanf("%s", s);
	// scanf("%lld%lld", &va, &vb, &vc, &vd);
	// scanf("%llu%llu%llu", &ua, &ub, &uc, &ud, &ue);
	scanf("%s", t);
	// scanf("%s", u);
	// scanf("%llu", &k);
	// scanf("%lld", &m);
	// for (i = 0; i < n; i++) {
	// 	scanf("%lld", &a[i]);
	// 	// scanf("%lld", &d[i]);
	// }
	// scanf("%llu", &q);
	// for (i = 0; i < n; i++) {
	// 	// scanf("%lld%lld", &xy[i].a, &xy[i].b);
	// 	// scanf("%lld%lld%lld", &tup[i].a, &tup[i].b, &tup[i].c);
	// 	// scanf("%lld", &c[i]);

	// 	scanf("%lld", &a[i]);
	// 	scanf("%lld", &b[i]);
	// 	scanf("%lld", &c[i]);
	// 	scanf("%lld", &d[i]);
	// 	// a[i]--;
	// 	// b[i]--;
	// 	// c[i]--;
	// 	// d[i]--;
	// 	// xy[i].a--;
	// 	// xy[i].b--;
	// 	// tup[i].a--;
	// 	// tup[i].b--;
	// }
	// for (i = 0; i < k; i++) {
	// 	// scanf("%lld%lld", &a[i], &b[i]);
	// 	// scanf("%lld", &b[i]);
	// 	// a[i]--;
	// 	// b[i]--;
	// 	scanf("%lld", &d[i]);
	// 	d[i]--;
	// }

	// for (i = 0; i < q; i++) {
	// 	// scanf("%lld%lld", &xy[i].a, &xy[i].b);
	// 	scanf("%lld", &c[i]);
	// 	// xy[i].a--;
	// 	// xy[i].b--;
	// }

	// for (i = 0; i < n; i++) {
	// 	for (j = 0; j < m; j++) {
	// 		scanf("%lld", &table[i][j]);
	// 		// table[i][j]--;
	// 	}
	// }
	// for (i = 0; i < n; i++) {
	// 	scanf("%s", s[i]);
	// }
	// scanf("%llu", &q);

	solve();

	return 0;
}
