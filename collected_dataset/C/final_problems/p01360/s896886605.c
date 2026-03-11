// AOJ 2243: Step Step Evolution
// 2017.11.11 bal4u@uu

#include <stdio.h>

#define LEFT  0
#define RIGHT 1
#define check(l,r)  ((tr[l])<=(tr[r]))

char a[100002];
int tr['9'+1];

int main()
{
	int i, s, cnt, ans;
	char *p, l, r;

	tr['1'] = tr['4'] = tr['7'] = 1;
	tr['2'] =           tr['8'] = 2;
	tr['3'] = tr['6'] = tr['9'] = 3;

	while (fgets(a, sizeof(a), stdin) && *a != '#') {
		if (a[1] < '0') { puts("0"); continue; }
		ans = 100000;
		for (i = 0; i < 2; i++) {
			cnt = 0, p = a;
			if (!i) l = *p++, r = *p++, s = LEFT;
			else    r = *p++, l = *p++, s = RIGHT;
			if (!check(l, r)) continue;

			while (*p >= '0') {
				if (s == LEFT) {
					if (check(*p, r)) l = *p, s = !s;
					else              r = *p, cnt++;
				} else {
					if (check(l, *p)) r = *p, s = !s;
					else              l = *p, cnt++;
				}
				p++;
			}
			if (ans > cnt) ans = cnt;
		}
		printf("%d\n", ans);
	}
	return 0;
}