// AOJ 1010: Dominoes Arrangement
// 2017.12.15 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char f['7'], g['7'];

int main()
{
	int n, i, k, ans;
	char buf[60], *p;

	while (fgets(buf, 6, stdin) != NULL) {
		n = atoi(buf);
		memset(f, 0, sizeof(f));
		memset(g, 0, sizeof(g));

		fgets(p=buf, 60, stdin);
		for (i = 0; i < n; i++) {
			if (*p == *(p+1)) g[*p] = 1;
			else f[*p]++, f[*(p+1)]++;
			p+=3;
		}

		ans = 0;
		if (n == 1) { ans = 1; goto done; }
		for (i = '0'; i <= '6'; i++) if (g[i] && !f[i]) goto done;
		k = 0; for (i = '0'; i <= '6'; i++) if (f[i] & 1) k++;
		ans = !k || k == 2;
done:	puts(ans ? "Yes" : "No");
	}
	return 0;
}