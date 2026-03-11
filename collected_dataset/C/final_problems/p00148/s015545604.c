#include <stdio.h>

int main(void)
{
	int a;

	while (scanf("%d", &a) != EOF){
		printf("3C");
		if(a % 39 == 0){
			printf("39");
		}
		else if (a - (a / 39) * 39 < 10){
			printf("0%d", a - (a / 39) * 39);
		}
		else {
			printf("%d", a - (a / 39) * 39);
		}
		printf("\n");
	}

	return (0);
}