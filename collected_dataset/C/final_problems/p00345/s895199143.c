#include <stdio.h>

int gcd(int a, int b) {
	int r;
	while (b > 0) {
		r = a % b;
		a = b;
		b = r;
	}
	return a;
}

int main(void) {
	char str[32];
	int zero_left, zero_right, one_left, one_right;
	int phase = 0;
	int i;
	int bunsi, bunbo, g;
	if (scanf("%s", str) != 1) return 1;
	zero_left = one_left = 1;
	zero_right = one_right = 0;
	for (i = 0; str[i] != '\0'; i++) {
		if (str[i] == '.') {
			phase = 1;
		} else if (str[i] == '(') {
			phase = 2;
		} else if (str[i] == ')') {
			phase = 3;
		} else {
			switch (phase) {
			case 0:
				zero_right = zero_right * 10 + (str[i] - '0');
				one_right = one_right * 10 + (str[i] - '0');
				break;
			case 1:
				zero_right = zero_right * 10 + (str[i] - '0');
				one_right = one_right * 10 + (str[i] - '0');
				zero_left *= 10;
				one_left *= 10;
				break;
			case 2:
				one_right = one_right * 10 + (str[i] - '0');
				one_left *= 10;
				break;
			case 3:
				break;
			}
		}
	}
	if (phase >= 2) {
		bunsi = one_right - zero_right;
		bunbo = one_left - zero_left;
	} else {
		bunsi = zero_right;
		bunbo = zero_left;
	}
	g = gcd(bunsi, bunbo);
	printf("%d/%d\n", bunsi / g, bunbo / g);
	return 0;
}