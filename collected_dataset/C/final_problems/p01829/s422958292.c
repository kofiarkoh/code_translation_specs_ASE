// AOJ 2728: Change a Password
// 2017.10.8 bal4u@uu

#include <stdio.h>
#include <string.h>

char s[11], t[11]; int n;
char c['9'+1];

int c2['9'+1]; int cc;
int check(int i, int a)
{
	int j, k;

	cc++;
	for (j = n-1; j >= i; j--) {
		t[j] = k = a % 10 + '0';
		if (c2[k] == cc || c[k]) return 0;
		c2[k] = cc;
		a /= 10;
	}
	return 1;
}

int main()
{
	int i, j, a, max;

	scanf("%s", s); n = strlen(s);
	if (s[0] >= '5') t[0] = s[0]-5; else t[0] = s[0]+5;
	c[t[0]] = 1;
	for (i = 1; i < n; i++) {
		a = s[i];
		if (c[a]) break;
		t[i] = a, c[a] = 1;
	}
	if (i < n) {
		max = a = 0, j = i;
		while (j < n) a = 10*a + (s[j++]-'0'), max = 10*max + 9;
		for (j = 1; ; j++) {
			if (a-j >= 0 && check(i, a-j)) break;
			if (a+j < max && check(i, a+j)) break;
		}
	}
	t[n] = 0;
	puts(t);
	return 0;
}