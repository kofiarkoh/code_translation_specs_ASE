#include <stdio.h>

int main(void)
{
	int num;
	double w, t, bmi;
	
	
	while (scanf("%d,%lf,%lf", &num, &w, &t) != EOF){
		
		bmi = w / (t * t);
		
		if (bmi >= 25){
			printf("%d\n", num);
		}
	}
	return (0);
}