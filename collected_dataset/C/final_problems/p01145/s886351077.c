// AOJ 2020: Princess's Japanese
// 2017.12.12 bal4u@uu

#include <stdio.h>
#include <string.h>

char bo['z'+1]; char *sbo = "aiueo";
char mu['z'+1]; char *smu = "ksthp";
char buf0[110], *buf = buf0 + 5;
char ans[103];

int main()
{
	int f, g;
	char *p, *q;

	p = sbo; while (*p) bo[*p++] = 1;
	p = smu; while (*p) mu[*p++] = 1;
	while (1) {
		memset(buf0, 0, sizeof(buf));
		fgets(buf, 103, stdin);
		if (*buf == '#') break;

		q = ans, f = 0;
		for (p = buf; *p > ' '; p++) {
            if (f && bo[*p]) { *q++ = *p, f = 0; continue; }
            g = mu[*(p-1)] || (*(p-1) == 'y' && (mu[*(p-2)]));
            if (g && (*p == 'i' || *p == 'u') && (*(p+1) < ' ' || mu[*(p+1)])) {
				*q++ = '(', *q++ = *p, *q++ = ')', f = 1; continue;
			}
            if (g && (*p == 'a' || *p == 'o') && mu[*(p+1)] && 
				(*(p+2) == *p || *(p+2) == 'y' && *(p+3) == *p)) {
				*q++ = '(', *q++ = *p, *q++ = ')', f = 1; continue;
            }
            *q++ = *p;
        }
		*q = 0;
        puts(ans);
    }
    return 0;
}