#include <stdio.h>

int main(void)
{
	int a, b;
	int i;
	int leap;
	
	while (scanf("%d %d", &a, &b) != EOF){
		leap = 0;
		if (a == 0 && b == 0){
			break;
		}
		for (i = a; i <= b; i++){
			if (i % 4 == 0 && i % 100 != 0 || i % 400 == 0){
				leap = i;
				printf("%d\n", leap);
			}
		}
		if (leap == 0){
			printf("NA\n");
		}
		printf("\n");
	}
	return (0);
}