// AOJ 2152 Restrictive Filesystem
// 2018.2.24 bal4u

#include <stdio.h>

#define INF 1000000010

typedef struct { int fid, from, end; } T;
T tbl[5000]; int sz;

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, i, j, cmd, id, sec, sno, from, end;

	while (n = in()) {
		tbl[0].fid = -1, tbl[0].from = 0, tbl[0].end = INF, sz = 1;

		while (n--) {
			cmd = getchar_unlocked(), getchar_unlocked();
			if (cmd == 'W') {
				id = in(), sec = in();
				for (i = 0; sec > 0; i++) {
					if (tbl[i].fid >= 0) continue;
					from = tbl[i].from, end = tbl[i].end;
					if (end-from+1 <= sec) {
						tbl[i].fid = id;
						sec -= end-from+1;
					} else {
						tbl[i].fid = id;
						tbl[i].from = from, tbl[i].end = from+sec-1;
						for (j = sz; j > i; j--) tbl[j] = tbl[j-1]; sz++;
						tbl[++i].fid = -1, tbl[i].from = from+sec, tbl[i].end = end;
						break;
					}
				}
			} else if (cmd == 'R') {
				sno = in();
				for (i = 0; i < sz; i++) {
					if (sno >= tbl[i].from && sno <= tbl[i].end) {
						printf("%d\n", tbl[i].fid);
						break;
					}
				}
			} else {		// cmd == 'D'
				id = in();
				for (i = 0; i < sz; i++) if (tbl[i].fid == id) tbl[i].fid = -1;
			}
		}
		putchar('\n');
	}
 	return 0;
}
