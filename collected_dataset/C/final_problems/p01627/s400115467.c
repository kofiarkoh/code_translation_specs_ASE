// AOJ 2515: Seishun 18 Kippu 2013
// 2017.10.5 bal4u@uu

#include <stdio.h>
#include <stdlib.h>

typedef struct { char nm[52]; int t[2]; } T;
T stn[11];

int main()
{
	int n, t, i, j, hh, mm, ans;
	char date[52];

	scanf("%d%d", &n, &t); n--;
	scanf("%s%s", date, date);
	for (i = 0; i < n; i++) for (j = 0; j < 2; j++) {
		scanf("%s%s", date, stn[i].nm);
		hh = atoi(date), mm = atoi(date+3);
		stn[i].t[j] = hh*60 + mm;
	}
	for (ans = 0, i = 0; i < n; i++)
		if (stn[i].t[1] - stn[i].t[0] >= t) ans++;

	printf("%d\n", ans);
	if (ans > 0) {
		for (i = 0; i < n; i++)
			if (stn[i].t[1] - stn[i].t[0] >= t)
				printf("%s %d\n", stn[i].nm, stn[i].t[1]-stn[i].t[0]);
	}
	return 0;
}