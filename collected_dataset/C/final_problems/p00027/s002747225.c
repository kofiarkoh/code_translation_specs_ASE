#include <stdio.h>

void day(int month, int today);

int main(void)
{
	int year, month;
	
	while (1){
		scanf("%d %d", &year, &month);
		if (year == 0 && month == 0) {
			break;
		}
		day(year, month);
	}
	
	return (0);
}

void day(int month, int today)
{
	int sa;
	int year;
	sa = 0;
	for (year = 1; year < month; year++) {
		if (year == 1 || year == 3 || year == 5 || year == 7 || year == 8 || year == 10 || year == 12) {
			sa += 31;
		}
		else if (year == 2) {
			sa += 29;
		}
		else {
			sa += 30;
		}
	}
	sa += today;
	
	if (sa % 7 == 0) {
		printf("Wednesday\n");
	}
	else if (sa % 7 == 1) {
		printf("Thursday\n");
	}
	else if (sa % 7 == 2) {
		printf("Friday\n");
	}
	else if (sa % 7 == 3) {
		printf("Saturday\n");
	}
	else if (sa % 7 == 4) {
		printf("Sunday\n");
	}
	else if (sa % 7 == 5) {
		printf("Monday\n");
	}
	else if (sa % 7 == 6) {
		printf("Tuesday\n");
	}
}
		