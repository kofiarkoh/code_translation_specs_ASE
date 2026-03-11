// AOJ 2728: Change a Password
// 2017.10.8 bal4u@uu

#include <stdio.h>
#include <string.h>

#define ABS(a) ((a)>=0?(a):-(a))
#define MIN(a,b) ((a)<=(b)?(a):(b))

char s[11], t[11], x[11]; int n;
long long  ns, pow10, diff;
char c['9'+1];

void try(int i, int mode)
{
	int j;
	long long a, b;

	if (i >= n) {
		for (a = 0, j = 0; j < n; j++) a = 10*a + (t[j]-'0');
		b = MIN(ABS(ns-a), pow10-ABS(ns-a));
		if (b > diff) diff = b, memcpy(x, t, sizeof(x));
		return;
	}
	if (mode) {
		for (j = '9'; c[j] && j >= '0'; j--);
		c[j] = 1; t[i] = j;
		try(i+1, mode);
	} else {
		for (j = '0'; c[j] && j <= '9'; j++);
		c[j] = 1; t[i] = j;
		try(i+1, mode);
	}
}

int main()
{
	int i, j;
	char *ans;

	scanf("%s", s); n = strlen(s);
	ns = 0, pow10 = 1; 
	for (i = 0; i < n; i++)
		ns = 10*ns + (s[i]-'0'), pow10 = 10*pow10;

	if (s[0] >= '5') t[0] = s[0]-5; else t[0] = s[0]+5;
	c[t[0]] = 1;
	for (i = 1; i < n; i++) {
		j = s[i];
		if (c[j]) break;
		t[i] = j, c[j] = 1;
	}

	if (i == n) ans = t;
	else {
		char c0['9'+1], t0[11]; int i0;
		diff = 0, ans = x, i0 = i;
		i0 = i, memcpy(c0, c, sizeof(c0)), memcpy(t0, t, sizeof(t0));
		for (j = s[i]-1; c[j] && j >= '0'; j--);
		if (j < '0') {
			for (j = i-1; j >= 0 && t[j] == '0'; j--) c['0'] = 0;
			i = j;
			for (c[t[i]] = 0, j = t[i]-1; c[j] && j >= '0'; j--);
		}
		c[j] = 1; t[i] = j;
		try(i+1, 1);

		i = i0, memcpy(c, c0, sizeof(c0)), memcpy(t, t0, sizeof(t0));
		for (j = s[i]+1; c[j] && j <= '9'; j++);
		if (j > '9') {
			for (j = i-1; j >= 0 && t[j] == '9'; j--) c['9'] = 0;
			i = j;
			for (c[t[i]] = 0, j = t[i]+1; c[j] && j <= '9'; j++);
		}
		c[j] = 1; t[i] = j;
		try(i+1, 0);
	}
	puts(ans);
	return 0;
}