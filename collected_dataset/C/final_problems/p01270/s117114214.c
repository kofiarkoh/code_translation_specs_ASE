// AOJ 2152 Restrictive Filesystem
// 2018.2.24 bal4u

#include <stdio.h>
#include <string.h>

#define INF 1000000010

typedef struct { int id, f, e, nxt; } T;
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
	int n, i, cmd, id, sno, from, end;

	while (n = in()) {
		tbl[0].id = -1, tbl[0].f = 0, tbl[0].e = INF, tbl[0].nxt = 0;
		sz = 1;

		while (n--) {
			cmd = getchar_unlocked(), getchar_unlocked();
			if (cmd == 'W') {
				id = in(), sno = in();
				for (i = 0; sno > 0; i = tbl[i].nxt) {
					if (tbl[i].id >= 0) continue;
					from = tbl[i].f, end = tbl[i].e;
					if (end-from+1 <= sno) {
						tbl[i].id = id;
						sno -= end-from+1;
					} else {
						tbl[i].id = id;
						tbl[i].f = from, tbl[i].e = from+sno-1;
						tbl[sz].nxt = tbl[i].nxt;
						tbl[i].nxt = sz;
						i = sz++;
						tbl[i].id = -1, tbl[i].f = from+sno, tbl[i].e = end;
						break;
					}
				}
			} else if (cmd == 'R') {
				sno = in();
				i = 0; while (sno < tbl[i].f || sno > tbl[i].e) i = tbl[i].nxt;
				printf("%d\n", tbl[i].id);
			} else {		// cmd == 'D'
				id = in();
				for (i = 0; tbl[i].nxt != 0; i = tbl[i].nxt)
					if (tbl[i].id == id) tbl[i].id = -1;
			}
		}
		putchar('\n');
	}
 	return 0;
}
