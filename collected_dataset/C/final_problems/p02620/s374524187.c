#include <stdio.h>
#include <stdlib.h>
#define D 5
#define INF 1001001001
typedef long long ll;
ll calcSat(int *tmpt, int s[][26], int *c) {
	int last[26] = {0};
	ll tmpSat = 0;
	for (int i = 0; i < D; i++) {
		tmpSat += s[i][tmpt[i] - 1];
		last[tmpt[i] - 1] = i + 1;
		for (int j = 0; j < 26; j++) {
			tmpSat -= (c[j] * (i + 1 - last[j]));
		}
		/**
		printf("%lld\n", tmpSat);
		/**/
	}
	return tmpSat;
}
int main() {
	srand(629);
	scanf("%*d");
	int c[26], s[D][26];
	int t[D] = {0};
	ll bestSat = 0;
	for (int i = 0; i < 26; i++) {
		scanf("%d", &c[i]);
	}
	for (int i = 0; i < D; i++) {
		for (int j = 0; j < 26; j++) {
			scanf("%d", &s[i][j]);
		}
	}
	/**/
	for (int i = 0; i < D; i++) {
		scanf("%d", &t[i]);
	}
	int m;
	scanf("%d", &m);
	for (int i = 0; i < m; i++) {
		int d, q;
		scanf("%d%d", &d, &q);
		t[d - 1] = q;
		ll tmpSat = calcSat(t, s, c);
		printf("%lld\n", tmpSat);
	}
	return 0;
}
