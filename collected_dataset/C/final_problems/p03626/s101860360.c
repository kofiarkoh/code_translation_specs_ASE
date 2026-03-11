#include <stdio.h>

#define MOD_BY 1000000007

int add(int a, int b) {
	return a + b - MOD_BY * (a + b >= MOD_BY);
}

int N;
char S1[64], S2[64];

int memo[64][4][4][4][4];

int sumipe(int pos, int ng1, int force1, int ng2, int force2) {
	int* m = &memo[pos][ng1][force1][ng2][force2];
	int i, j;
	int answer = 0;
	if (pos >= N) return 1;
	if (*m) return ~*m;
	if (S1[pos] == S2[pos]) {
		for (i = 1; i <= 3; i++) {
			if (i != ng1 && i != ng2) {
				answer = add(answer, sumipe(pos + 1, i, 0, i, 0));
			}
		}
	} else {
		for (i = 1; i <= 3; i++) {
			if (i != ng1 && (force1 == 0 || i == force1)) {
				for (j = 1; j <= 3; j++) {
					if (j != ng2 && i != j && (force2 == 0 || j == force2)) {
						int f1 = S1[pos] == S1[pos + 1], f2 = S2[pos] == S2[pos + 1];
						answer = add(answer, sumipe(pos + 1, f1 ? 0 : i, f1 ? i : 0, f2 ? 0 : j, f2 ? j : 0));
					}
				}
			}
		}
	}
	return ~(*m = ~answer);
}

int main(void) {
	if (scanf("%d", &N) != 1) return 1;
	if (scanf("%63s", S1) != 1) return 1;
	if (scanf("%63s", S2) != 1) return 1;
	printf("%d\n", sumipe(0, 0, 0, 0, 0));
	return 0;
}
