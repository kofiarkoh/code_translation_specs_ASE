// AOJ 2614: Almost Same Substring
// 2017.12.31 bal4u@uu

#include <stdio.h>
#include <string.h>

int slen, tlen, diff;
char s[300005];
char t[300005];
int zf[600005];
int zr[600005];
char buf[600005];

void Zalgo(int *z, char *s, int n)
{
	int i, j, k;

	z[0] = n;
    i = 1, j = 0;
    while (i < n) {
		while (i+j < n && s[j] == s[i+j]) ++j;
        z[i] = j;
        if (j == 0) { ++i; continue;}
        k = 1;
        while (i+k < n && k+z[k] < j) z[i+k] = z[k], ++k;
        i += k; j -= k;
    }
}

int main()
{
	int i, w, ans;
	char *p, *q;

	fgets(s, sizeof(s), stdin);
	slen = strlen(s)-1;
	if (*(s+slen) < ' ') *(s+slen) = 0;	else slen++;
	fgets(t, sizeof(t), stdin);
	tlen = strlen(t)-1;
	if (*(t+tlen) < ' ') *(t+tlen) = 0;	else tlen++;

	memcpy(buf, t, tlen);
	memcpy(buf+tlen, s, slen);
	Zalgo(zf, buf, slen+tlen+1);

	p = buf, q = t+tlen;
	while (q > t) *p++ = *--q;
	q = s+slen;
	while (q > s) *p++ = *--q;
	Zalgo(zr, buf, slen+tlen+1);

	ans = 0, w = tlen-1;
	for (i = tlen; i <= slen; i++) {
		ans += (zf[i]+ zr[slen+tlen-i] == w);
	}
	printf("%d\n", ans);

	return 0;
}