#include <stdio.h>

int fact(int n)
{
	int i = 0, ret = 1;

	for (i = 1; i <= n; i++) {
		ret *= i;
	}
	return ret;
}

int main()
{
	char buf[256];
	int i, n;
	int count[64] = {0};
	int c, sum;

	scanf("%s", buf);
	for (i = 0; buf[i] != '\0'; i++) {
		count[buf[i] - 'a']++;
	}
	c = 0;
	sum = 0;
	for (i = 0; i < 26; i++) {
		if (count[i] % 2 == 1) {
			c++;
			if (c >= 2) {
				puts("0");
				return 0;
			}
			if (count[i] == 1) {
				count[i] = 0;
			}
		} else {
			count[i] /= 2;
			sum += count[i];
		}
	}
	sum = fact(sum);
	for (i = 0; i < 26; i++) {
		if (count[i] != 0) {
			sum /= fact(count[i]);
		}
	}
	printf("%d\n", sum);
	return 0;
}