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

ull n, m;
ull umin (ull x, ull y) {
	return (x < y) ? x : y;
}

sll a[N_MAX + 5];
sll b[N_MAX + 5];

ull g[305][305];
bool seen[305];

ull dfs (ull v, ull u, ull f) {
	seen[v] = true;
	if (v == u) return f;

	for (sll i = 0; i < 305; i++) {
		if (seen[i] || !g[v][i]) continue;
		ull l = dfs(i, u, umin(f, g[v][i]));
		if (!l) continue;

		g[v][i] -= l;
		g[i][v] += l;
		return l;
	}
	
	return 0;
}

ull flow (ull s, ull g) {
	ull total = 0;
	while (true) {
		for (sll i = 0; i < 305; i++) seen[i] = false;

		ull f = dfs(s, g, VERYBIG);
		if (!f) return total;
		total += f;
	}
}

ull solve () {
	sll i, j, ki, li;
	// ull result = 0;
	sll result = 0;

	ull x = 300;
	for (i = 1; i <= x; i++) {
		for (j = 2; j * i <= x; j++) {
			g[i][i * j] = VERYBIG;
		}
	}

	sll best = 0;
	for (i = 0; i < n; i++) {
		if (b[i] > 0) {
			best += b[i];
			g[a[i]][x + 1] += b[i];
		} else {
			g[0][a[i]] += -b[i];
		}
	}

	result = best - flow(0, x + 1);

	printf("%lld\n", result);
	return 0;
}

int32_t main (void) {
	int32_t i, j;

	scanf("%llu", &n, &m);
	for (i = 0; i < n; i++) {
		scanf("%lld", &a[i]);
		scanf("%lld", &b[i]);
	}

	solve();

	return 0;
}
