#include <stdio.h>

int main(void)
{
	float a, b, i;
	
	
	while (scanf("%f %f",&a ,&b) != EOF){
		if (35.50 > a && 71.00 > b){
			printf("AAA\n");
		}
		else if (37.50 > a && 77.00 > b){
			printf("AA\n");
		}
		else if (40.00 > a && 83.00 > b){
			printf("A\n");
		}
		else if (43.00 > a && 89.00 > b){
			printf("B\n");
		}
		else if (50.00 > a && 105.0 > b){
			printf("C\n");
		}
		else if (55.00 > a && 116.0 > b){
			printf("D\n");
		}
		else if (70.00 > a && 148.0 > b){
			printf("E\n");
		}
		else {
			printf("NA\n");
	}
	}
	
	return (0);
}