#include<stdio.h>
#include<time.h>
#include<string.h>
int main(void) {
	time_t tim1,tim2;
	struct tm d;
	double diff;
	int year1, year2, month1, month2, day1, day2;
	while (1) {
		scanf("%d %d %d %d %d %d",&year1,&month1,&day1,&year2,&month2,&day2);
		if (month1 == -1 || year1 == -1 || day1 == -1 ||month2 == -1 || year2 == -1 || day2 == -1) break;
		memset(&d,0,sizeof(d));
		d.tm_year = year1 - 1900;
		d.tm_mon = month1 - 1;
		d.tm_mday = day1;
		tim1 = mktime(&d);
		d.tm_year = year2 - 1900;
		d.tm_mon = month2 - 1;
		d.tm_mday = day2;
		tim2 = mktime(&d);

		diff = difftime(tim2, tim1);

		printf("%d\n",(int)(diff/60/60/24+0.5));
	}
	return 0;
}