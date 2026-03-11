#include <stdio.h>

int main(void)
{
	double time1;
	double time2;
	
	while(scanf("%lf %lf", &time1, &time2) != EOF){
		if (time1 < 35.5 && time2 < 71){
			printf("AAA\n");
		}
		else if (time1 < 37.5 && time2 < 77){
			printf("AA\n");
		}
		else if (time1 < 40 && time2 < 83){
			printf("A\n");
		}
		else if (time1 < 43 && time2 < 89){
			printf("B\n");
		}
		else if (time1 < 50 && time2 < 105){
			printf("C\n");
		}
		else if (time1 < 55 && time2 < 116){
			printf("D\n");
		}
		else if (time1 < 70 && time2 < 148){
			printf("E\n");
		}
		else {
			printf("NA\n");
		}
	}
	
	return (0);
}