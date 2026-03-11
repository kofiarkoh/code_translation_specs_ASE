#include <stdio.h>

int main(void)
{
	double m, n;

	while (scanf("%lf %lf", &m, &n) != EOF)
	{
		if (m < 35.5 && n < 71)
			printf("AAA\n");
		else if (m < 37.5 && n < 77)
			printf("AA\n");
		else if (m < 40 && n < 83)
			printf("A\n");
		else if (m < 43 && n < 89)
			printf("B\n");
		else if (m < 50 && n < 105)
			printf("C\n");
		else if (m < 55 && n < 116)
			printf("D\n");
		else if (m < 70 && n < 148)
			printf("E\n");
		else
			printf("NA\n");
	}

	return 0;
}