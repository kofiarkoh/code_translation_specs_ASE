// AOJ 2728: Change a Password
// 2017.10.8 bal4u@uu

#include <stdio.h>
#include <string.h>

char s[11], t[11];
char c['9'+1];

int main()
{
	int n, i, j, k, f;

	scanf("%s", s); n = strlen(s);
	if (s[0] >= '5') t[0] = s[0]-5; else t[0] = s[0]+5;
	c[t[0]] = 1;

	for (f = 0, i = 1; i < n; i++) {
		if      (f == 1) for (j = '9'; c[j]; j--);
		else if (f == 2) for (j = '0'; c[j]; j++);
		else if (c[s[i]]) {
			for (k = 1; ; k++) {
				int j1, j2;
				j1 = s[i]-k; if (j1-k < '0') j1 = s[i];
				j2 = s[i]+k; if (j2+k > '9') j2 = s[i];
				if (c[j1] && c[j2]) continue;
				if (!c[j1]) j = j1;
				if (!c[j2] && (c[j1] || j2 < j1)) j = j2;
				break;
			}
			if (j < s[i]) f = 1; else f = 2;
		} else j = s[i];
		t[i] = j, c[j] = 1;
	}
	t[n] = 0;
	puts(t);
	return 0;
}