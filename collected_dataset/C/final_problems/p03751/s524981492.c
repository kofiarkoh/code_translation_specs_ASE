/*
cat <<EOF >mistaken-paste
*/

#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

#define BIG 2000000007
#define VERYBIG 200000000000007LL

#define MOD 1000000007
typedef uint64_t ull;
typedef  int64_t sll;

#define N_MAX 10000
#define M_MAX 400

#ifdef __cplusplus
#include <queue>
#include <stack>
#include <tuple>

using namespace std; // I'm NOT gonna use C++ without compro. shit

typedef priority_queue<ull, vector<ull>, greater<ull> > upque123;
typedef priority_queue<ull, vector<ull> > upque321;
typedef priority_queue<sll, vector<sll>, greater<sll> > spque123;
typedef priority_queue<sll, vector<sll> > spque321;

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


int32_t charcomp (const char left, const char right) {
	if (left < right) {
		return -1;
	} else if (left > right) {
		return +1;
	} else {
		return 0;
	}
}


sll dist[N_MAX];
struct_b path[M_MAX * 2];

char s[N_MAX + 1][M_MAX + 1];
char t[N_MAX + 1];

ull alphabets[26];

bool isfaster[N_MAX];
bool islater[N_MAX];

double distance (sll x1, sll y1, sll x2, sll y2) {
	double xdist2, ydist2, origindist, dist;

	xdist2 = (x1 - x2) * (x1 - x2);
	ydist2 = (y1 - y2) * (y1 - y2);
	return sqrt(xdist2 + ydist2);
}

ull solve () {
	sll i, j, ki, l;
	ull result = 0;
	ull maybe = 0;
	ull sum = 0;
	ull item;
	ull *dpcell;
	// qsortの際には"p"ullcompを使う

	for (i = 0; i < n; i++) {
		j = 0;
		while (true) {
			if (s[i][j] == '?') {
				if (!t[j]) {
					isfaster[i] = true;
					break;
				}

				if (t[j] == 'a') {
					isfaster[j] = true;
				} else if (t[j] == 'z') {
					islater[j] = true;
				} else {
					isfaster[i] = islater[i] = true;
					break;
				}
			} else {
				int c = charcomp(s[i][j], t[j]);
				if (c > 0) {
					isfaster[i] = true;
					break;
				}
				if (c < 0) {
					islater[i] = true;
					break;
				}
				if (!s[i][j] && !t[j]) {
					isfaster[i] = islater[i] = true;
					break;
				}
			}

			j++;
		}
	}

	ull minval = 1, maxval = n + 1;
	for (i = 0; i < n; i++) {
		if (!isfaster[i]) minval++;
		if (!islater[i]) maxval--;
	}

	for (i = minval; i <= maxval; i++) {
		printf("%llu%c", i, (i == maxval ? '\n' : ' '));
	}


	return 0;
}

int32_t main (void) {
	int32_t i, j;
	int32_t x, y;

	scanf("%llu", &n, &m);
	for (i = 0; i < n; i++) {
		scanf("%s", s[i]);
	}
	scanf("%s", t);

	solve();

	return 0;
}
