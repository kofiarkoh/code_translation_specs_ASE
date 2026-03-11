// AOJ 1195: Encryption System
// 2017.12.8 bal4u@uu

#include <stdio.h>
#include <string.h>

int n;
char ans[50000][22];
char f['z'+1];
char buf[22], tmp[22], *p;

void rec(char *p, char *s)
{
	int t;
	char c;

	c = *p++;
	if (c < 'a') {
		memcpy(ans[n], tmp, s-tmp), ans[n++][s-tmp] = 0;
		return;
	}

	if (f[c] || c == 'a') {
		t = f[c];
		f[c] = 1;
		*s = c;
		rec(p, s+1);
		f[c] = t;
	}
	if (!f[c+1] && c != 'z') {
		t = f[c+1];
		f[c+1] = 1;
		*s = c+1;
		rec(p, s+1);
		f[c+1] = t;
	}
}

int main()
{
	int i;

	while (fgets(p=buf, 22, stdin) && *p != '#') {
		memset(f, 0, sizeof(f));
		n = 0;
		rec(p, tmp);
		printf("%d\n", n);

		if (n <= 10) {
			for (i = 0; i < n; i++) puts(ans[i]);
		} else {
			for (i = 0; i < 5; i++) puts(ans[i]);
			for (i = n-5; i < n; i++) puts(ans[i]);
		}
	}
	return 0;
}