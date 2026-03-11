// Aizu Vol-2 0225: Kobutanukitsuneko
// 2017.8.7

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *gets();

char buf[50];
int s[26], e[26];

int main()
{
	int n, a, b, w, f;
	
	while (1) {
		gets(buf), n = atoi(buf);
		if (n == 0) break;
		f = 0;
		memset(s, 0, sizeof(s)), memset(e, 0, sizeof(e));
		while (n-- > 0) {
			gets(buf);
			w = strlen(buf); if (w == 1) f = 1;
			a = buf[0] - 'a', b = buf[w-1] - 'a';
			s[a]++, e[b]++;
		}
		if (f) { puts("NG"); continue; }
		for (n = 0; ; n++) {
			if (n >= 26) { puts("OK"); break; }
			if (s[n] != e[n]) { puts("NG"); break; }
		}
	}
	return 0;
}