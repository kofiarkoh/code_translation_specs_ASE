// AOJ 1215: Co-occurrence Search
// 2017.10.15

#include <stdio.h>
#include <string.h>

char s[1000003];
char *pos[55]; int key[128], len;

int gettext()
{
	char c, *p = s;
	while (1) {
		if ((c = getchar()) < 0 || (c == '\n' && p == s)) return 0;
		if (c == '\n') if ((c = getchar()) < 0 || c == '\n') { *p = 0; break; }
		*p++ = c;
	}
	return 1;
}

void print(char *s, char *e)
{
	int w = 0;
	while (s <= e) {
		if (w == 72) putchar('\n'), w = 0;
		putchar(*s++), w++;
	}
	puts("\n");
}

int check(char *p)
{
	int i;
	char *q = p;
	for (i = 1; i <= len; i++) if (pos[i] < q) q = pos[i];
	return p-q;
}

int main()
{
	int k, t, min, ans, cnt;
	char buf[80], *p, *pp;

	while (1) {
		if (!gettext()) break;
		memset(key, 0, sizeof(key)), memset(pos, 0, sizeof(pos));
		fgets(p = buf, 80, stdin);
		len = 0; while (*p > ' ') key[*p++] = ++len;

		min = 0x7fffffff, ans = cnt = 0, p = s;
		while (*p) {
			if ((t = key[*p]) > 0) {
				if (pos[t] == NULL) cnt++;
				pos[t] = p;
				if (cnt == len) {
					if ((k = check(p)) < min) min = k, ans = 1, pp = p-k;
					else if (k == min) ans++;
				}
			}
			p++;
		}
		printf("%d\n\n", ans);
		if (ans > 0) print(pp, pp+min);
		fgets(buf, 80, stdin);		// skip empty line
	}
	return 0;
}