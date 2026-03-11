// AOJ 2030: Ruins
// 2017.10.1 bal4u@uu

#include <stdio.h>
#include <math.h>

int main()
{
	int a, b, sa, sb;
	int a1, a2, b1, b2;
	int t1, t2, t3, t4, t, ans;

	while (scanf("%d%d", &a, &b) && a > 0) {

		sa = (int)sqrt((double)a)+1, sb = (int)sqrt((double)b)+1;
		while (sa*sa > a) sa--; while (sb*sb > b) sb--;

		for (ans = 0x7fffffff, a1 = 1; a1 <= sa; a1++) {
			if (a % a1) continue;
			a2 = a / a1;
			for (b1 = 1; b1 <= sb; b1++) {
				if (b % b1) continue;
				b2 = b / b1;

				if (a1 > b1) t1 = b1, t2 = a1;
				else         t1 = a1, t2 = b1;
				if (a2 > b2) t3 = b2, t4 = a2;
				else         t3 = a2, t4 = b2;
				if (t2 > t3) t = t2, t2 = t3, t3 = t;
				t = (t2-t1)*(t2-t1)+(t3-t2)*(t3-t2)+(t4-t3)*(t4-t3);
				if (t < ans) ans = t;
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}