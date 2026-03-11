// AOJ 2728: Change a Password
// 2017.10.8 bal4u@uu

#include <stdio.h>
#include <string.h>

char s[11], t[11];
char c['9'+1];

int main()
{
	int n, i, j, f;

	scanf("%s", s); n = strlen(s);
	if (s[0] >= '5') t[0] = s[0]-5; else t[0] = s[0]+5;
	c[t[0]] = 1;

	for (f = 0, i = 1; i < n; i++) {
		if      (f == 1) for (j = '9'; c[j]; j--);
		else if (f == 2) for (j = '0'; c[j]; j++);
		else if (c[s[i]]) {
			if (s[i] > '0') {
				f = 1; for (j = s[i]-1; c[j]; j--);
				if (j < '0') {
					f = 2; for (j = '0'; c[j]; j++);
				}
			} else { f = 2; for (j = s[i]+1; c[j]; j++); }
		} else j = s[i];
		t[i] = j, c[j] = 1;
	}
	t[n] = 0;
	puts(t);
	return 0;
}