// Aizu 0235: Sergeant Rian
// 2017.9.11 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { char f; int len, to[22], t[22]; } T;
T tbl[22];
int n;

int mostfar(int k)
{
	int i, j, max, s;

	if (tbl[k].f) return 0;
	tbl[k].f = 1;
	for (max = 0, i = 0; i < tbl[k].len; i++) {
		if ((j = tbl[k].to[i]) >= 0 && tbl[j].f == 0) {
			s = tbl[k].t[i] + mostfar(j);
			if (s > max) max = s;
		}
	}
	return max;
}

int main()
{
	int i, j, a, b, t, ans;
	T *p;

	while (scanf("%d", &n) && n) {
		memset(tbl, 0, sizeof(tbl));
		for (ans = 0, i = 1; i < n; i++) {
			scanf("%d%d%d", &a, &b, &t), a--, b--;
			p = tbl + a, p->to[p->len] = b, p->t[p->len++] = t;
			p = tbl + b, p->to[p->len] = a, p->t[p->len++] = t;
			ans += t;
		}
		for (i = 1; i < n; i++) if (tbl[i].len == 1) {
			p = tbl + tbl[i].to[0];
			for (j = 0; j < p->len; j++) 
				if (p->to[j] == i) { ans -= p->t[j], p->to[j] = -1; break; }
		}
		printf("%d\n", 2*ans - mostfar(0));
	}
	return 0;
}