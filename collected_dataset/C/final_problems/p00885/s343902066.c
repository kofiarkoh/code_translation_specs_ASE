#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INF 999999999

int n;
int p[64], t[64];

int memo[64][4];

int search(int idx, int prevStock) {
	int answer = INF + n - idx;
	int zikoku;
	int i;
	int prevPos;
	int cost;
	if (idx > n) return 0;
	if (memo[idx][prevStock]) return ~memo[idx][prevStock];

	zikoku = t[idx - 1] + (prevStock + 1) * p[idx - 1];
	prevPos = 0;
	cost = 0;
	for (i = 0; i < 3 && idx + i <= n; i++) {
		int deltaCost = abs(p[idx + i] - prevPos);
		int nextZikoku = zikoku + (i + 1) * deltaCost;
		cost += deltaCost;
		if (nextZikoku <= t[idx + i]) {
			if (idx + i + 1 <= n && INF + n - (idx + i + 1) < answer) answer = INF + n - (idx + i + 1);
			if (idx + i + 1 > n || t[idx + i] + (i + 1 + 1) * p[idx + i] <= t[idx + i + 1]) {
				int candidate = search(idx + i + 1, i + 1);
				if (candidate < INF) candidate += cost + p[idx + i];
				if (candidate < answer) answer = candidate;
			}
		} else {
			break;
		}
		prevPos = p[idx + i];
		zikoku = t[idx + i];
	}

	return ~(memo[idx][prevStock] = ~answer);
}

int main(void) {
	while (scanf("%d", &n) == 1 && n > 0) {
		int i;
		int answer;
		p[0] = 0;
		t[0] = 0;
		for (i = 1; i <= n; i++) {
			if (scanf("%d%d", &p[i], &t[i]) != 2) return 1;
		}
		memset(memo, 0, sizeof(memo));
		answer = search(1, 0);
		if (answer < INF) {
			printf("OK %d\n", answer);
		} else {
			printf("NG %d\n", INF + n - answer);
		}
	}
	return 0;
}

