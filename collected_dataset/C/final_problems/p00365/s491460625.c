// AOJ 0370: Age Difference
// 2017.11.14 bal4u@uu

#include <stdio.h>

int main()
{
	int y1, m1, d1, y2, m2, d2, x, f;

	scanf("%d%d%d%d%d%d", &y1, &m1, &d1, &y2, &m2, &d2);
	if (y1 > y2) {
		x = y1, y1 = y2, y2 = x;
		x = m1, m1 = m2, m2 = x;
		x = d1, d1 = d2, d2 = x;
	}
	f = 0;
	if (m1 < m2 || m1 == m2 && d1 < d2) f = 1;
	if (y1 == y2 && (m1 != m2 || d1 != d2)) f = 1;
	printf("%d\n", y2-y1+f);
	return 0;
}