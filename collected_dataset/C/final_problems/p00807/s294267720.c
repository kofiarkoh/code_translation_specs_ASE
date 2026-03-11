// AOJ 1228: Beehives
// 2017.12.5 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BASE 128
char t[102], h[102];
int pos[102], n;
int mv[6][2] = {{1,0},{1,1},{0,1},{-1,0},{-1,-1},{0,-1}};
char chk[65536];

int main()
{
	int cno, i, k, r, c, d, e, ans;
	char buf[10], *p;

	fgets(buf, 10, stdin), cno = atoi(buf);
	while (cno--) {
		memset(chk, 0, sizeof(chk));

		fgets(t, 102, stdin);
		fgets(h, 102, stdin);
		fgets(buf, 10, stdin);
	
		ans = 0;
		if (strlen(t) != strlen(h)) goto done;
		r = c = BASE;
		pos[0] = k = (r << 8) | c, chk[k] = 1;
		n = 1;
		for (p = t; *p >= 'a'; p++) {
			d = (*p & 0xf) - 1;
			r += mv[d][0], c += mv[d][1];
			pos[n++] = k = (r << 8) | c, chk[k] = 1;
		}

		for (i = 0; i < n; i++) for (d = 0; d < 6; d++) {
			r = pos[i] >> 8, c = pos[i] & 0xff;
			for (p = h; ; p++) {
				if (*p < 'a') { ans = 1; goto done; }
				e = d + ((*p & 0xf) - 1);
				if (e >= 6) e -= 6;
				r += mv[e][0], c += mv[e][1];
				k = (r << 8) | c;
				if (!chk[k]) break;
			}
		}
done:	puts(ans ? "true" : "false");
	}
	return 0;
}