#include <stdio.h>
#include <stdlib.h>

int n;
char raw_data[1000][128];
int kaisou[1000];
char* data[1000];

int calc(int pos) {
	if (data[pos][0] == '+' || data[pos][0] == '*') {
		int ret = (data[pos][0] == '+' ? 0 : 1);
		int i;
		for (i = pos + 1; i < n && kaisou[i] > kaisou[pos]; i++) {
			if (kaisou[i] == kaisou[pos] + 1) {
				if (data[pos][0] == '+') ret += calc(i);
				else ret *= calc(i);
			}
		}
		return ret;
	} else {
		return atoi(data[pos]);
	}
}

int main(void) {
	while (scanf("%d", &n) == 1 && n > 0) {
		int i, j;
		for (i = 0; i < n; i++) {
			if (scanf("%s", raw_data[i]) != 1) return 1;
			for (j = 0; raw_data[i][j] == '.'; j++);
			kaisou[i] = j;
			data[i] = raw_data[i] + j;
		}
		printf("%d\n", calc(0));
	}
	return 0;
}