// AOJ 2881: Change of the Era Name
// 2018.7.11 bal4u

#include <stdio.h>

int new = (31<<9) + (5<<5) + 1;		// H31.5.1

int main()
{
	int y, m, d, date;
	char era[10], buf[50];
	
	while (fgets(buf, 50, stdin) && *buf != '#') {
		sscanf(buf, "%s%d%d%d", era, &y, &m, &d);
		date = (y<<9) + (m<<5) + d;
		if (date >= new) printf("? %d %d %d\n", y-30, m, d);
		else printf("%s %d %d %d\n", era, y, m, d);
	}
	return 0;
}

