#include <stdio.h>

#define INF 9999999

int n, m;
int c[10];

int memo[6][512];

int search(int leftMaisu, int leftKane) {
	int answer = INF;
	int i;
	if (leftKane < 0) return INF;
	if (leftMaisu <= 0) return 0;
	if (memo[leftMaisu][leftKane]) return ~memo[leftMaisu][leftKane];
	for (i = 0; i <= 9; i++) {
		if (c[i] <= leftKane) {
			int candidate = search(leftMaisu - 1, leftKane - c[i]) * 10 + i;
			if (candidate < answer) answer = candidate;
		}
	}
	return ~(memo[leftMaisu][leftKane] = ~answer);
}

int main(void) {
	int i;
	int kotae;
	if (scanf("%d%d", &n, &m) != 2) return 1;
	for (i = 0; i < 10; i++) {
		if (scanf("%d", &c[i]) != 1) return 1;
	}
	kotae = search(n, m);
	if (kotae < INF) {
		printf("%0*d\n", n, kotae);
	} else {
		puts("NA");
	}
	return 0;
}

