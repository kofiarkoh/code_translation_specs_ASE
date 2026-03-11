// AOJ 1518: Last One
// 2018.1.17 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

char buf[15];

int main()
{
	int n, m;
	int ans;
	char *p;

	fgets(buf, 15, stdin), n = atoi(buf);
	ans = 0;
	while (n--) {
		fgets(p=buf, 15, stdin);
		while (isdigit(*p)) p++;
		m = 0; while (*++p > ' ') {
			if      (isdigit(*p)) m += *p - '0';
			else if (isupper(*p)) m += *p - 'A' + 10;
			else                  m += *p - 'a' + 36;
		}
		ans ^= m;
	}
	puts(ans? "win": "lose");
	return 0;
}
