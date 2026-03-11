// AOJ 1554: Friday the 13th
// 2019.2.22 bal4u
 
#include <stdio.h>
#include <stdlib.h>

#define MAGIC 6880  // 1000/1/1-4999/12/31
#define Magic 1719  // 1/1/1-999/12/31

int dayOfWeek(int year, int month, int day)
{
	if (month == 1 || month == 2) year--, month += 12;
	return (year + year / 4 - year / 100 + year / 400
		+ (13 * month + 8) / 5 + day) % 7;
}

long long dayFri13(long long year, int month, int day)
{
	int y, m;
	long long ans;

	year += 3000;
	ans = Magic + year / 4000 * MAGIC;
	year = 1000 + year % 4000;
	for (y = 1000; y < year; y++) {
		for (m = 1; m <= 12; m++)
			if (dayOfWeek(y, m, 13) == 5) ans++;
	}
	if (month > 1 || (month == 1 && day >= 13)) {
		if (day < 13) month--;
		for (m = 1; m <= month; m++)
			if (dayOfWeek(y, m, 13) == 5) ans++;
	}
	return ans;
}

int main()
{
	long long y1, y2; int m1, m2, d1, d2;

	scanf("%lld%d%d%lld%d%d", &y1, &m1, &d1, &y2, &m2, &d2);
	if (d1 == 13) d1--;
	printf("%lld\n", dayFri13(y2, m2, d2) - dayFri13(y1, m1, d1));
	return 0;
}
