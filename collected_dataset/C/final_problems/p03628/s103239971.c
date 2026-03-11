#include <stdio.h>

int main(void) {

  int n;
	scanf("%d", &n);
	char s[2][n+1];
	scanf("%s %s", s[0], s[1]);
	long mod = 1000000007;
	int prev = -1;
	long answer = 1;
	for (int i = 0; i < n; i++) {
		if (i != n-1 && s[0][i] == s[0][i+1]) {
			i++;
			if (prev == -1) {
				answer *= 3*2;
			} else if (prev == 1) {
				answer *= 2;
			} else {
				answer *= 3;
			}
			answer %= mod;
			prev = 2;
		} else {
			if (prev == -1) {
				answer *= 3;
			} else if (prev == 1) {
				answer *= 2;
			} else {
				answer *= 1;
			}
			answer %= mod;
			prev = 1;
		}
	}
	printf("%ld\n", answer);

  return 0;
}