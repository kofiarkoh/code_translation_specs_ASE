// AOJ 1054: Distorted Love
// 2017.9.28 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { int x1, y1, x2, y2; char nm[25]; int pag; } BTN;
typedef struct { char nm[25]; int id, next, prev; int bn; BTN btn[102]; } PAG;
PAG pag[200]; int n;

int now, h, w;

int whatPage(char *nm)
{
	int i;
	for (i = 0; i < n; i++) if (strcmp(nm, pag[i].nm) == 0) return i;
	while (1);   // panic
	return -1;
}

int main()
{
	int m, i, j, k, x, y;
	PAG *pp, *new;
	char cmd[20];

	while (scanf("%d", &n) && n > 0) {
		scanf("%d%d", &w, &h);
		for (pp = pag, i = 0; i < n; i++, pp++) {
			scanf("%s%d", pp->nm, &k), pp->bn = k, pp->id = i, pp->next = pp->prev = -1;
			for (j = 0; j < k; j++)
				scanf("%d%d%d%d%s", &pp->btn[j].x1, &pp->btn[j].y1,
					  &pp->btn[j].x2, &pp->btn[j].y2, &pp->btn[j].nm);
		}
		for (pp = pag, i = 0; i < n; i++, pp++)
			for (j = 0; j < pp->bn; j++) pp->btn[j].pag = whatPage(pp->btn[j].nm);

		now = 0;
		scanf("%d", &m);
		while (m-- > 0) {
			scanf("%s", cmd);
			if (*cmd == 'b') {
				pp = pag + now;
				if (pp->prev >= 0) now = pag[pp->prev].id;
			} else if (*cmd == 'f') {
				pp = pag + now;
				if (pp->next >= 0) now = pag[pp->next].id;
			} else if (*cmd == 's') puts(pag[now].nm);
			else {		// click
				scanf("%d%d", &x, &y);
				pp = pag + now;
				for (j = 0; j < pp->bn; j++) {
					if (pp->btn[j].x1 <= x && x <= pp->btn[j].x2 &&
						pp->btn[j].y1 <= y && y <= pp->btn[j].y2) {
						new = pag + pp->btn[j].pag;
						new->prev = pp->id, new->next = -1;
						pp->next = new->id;
						now = new->id;
						break;
					}
				}
			}
		}
	}
	return 0;
}