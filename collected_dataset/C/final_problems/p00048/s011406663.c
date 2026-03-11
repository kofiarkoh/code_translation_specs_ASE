#include <stdio.h>

int main(void)
{
	double weight;
	
	while (scanf("%lf",&weight) != EOF){
		if (weight > 91.00){
			printf("heavy\n");
		}
		else if (weight > 81.00){
			printf("light heavy\n");
		}
		else if (weight > 75.00){
			printf("middle\n");
		}
		else if (weight > 69.00){
			printf("light middle\n");
		}
		else if (weight > 64.00){
			printf("welter\n");
		}
		else if (weight > 60.00){
			printf("light welter\n");
		}
		else if (weight > 57.00){
			printf("light\n");
		}
		else if (weight > 54.00){
			printf("feather\n");
		}
		else if (weight > 51.00){
			printf("bantam\n");
		}
		else if (weight > 48.00){
			printf("fly\n");
		}
		else {
			printf("light fly\n");
		}
	}
	
	return (0);
}