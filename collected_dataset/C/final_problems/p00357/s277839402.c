#include <stdio.h>

int N;
int d[300000];

int main(void) {
	int i;
	int ok_max = 0;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &d[i]) != 1) return 1;
	}
	for (i = 0; i < N && i <= ok_max; i++) {
		int this_max = i + d[i] / 10;
		if (this_max > ok_max) ok_max = this_max;
	}
	if (i < N) {
		puts("no");
		return 0;
	}
	ok_max = N - 1;
	for (i = N - 1; i >= 0 && i >= ok_max; i--) {
		int this_max = i - d[i] / 10;
		if (this_max < ok_max) ok_max = this_max;
	}
	if (i >= 0) {
		puts("no");
		return 0;
	}
	puts("yes");
	return 0;
}

