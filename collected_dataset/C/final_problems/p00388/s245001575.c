#include <stdio.h>

int main()
{
	int H, A, B, i, Pattern;

	scanf("%d%d%d", &H, &A, &B);
	for (i = A; i <= B; i++)
	{
		if (H % i == 0)
		{
		    Pattern++;
		}
	}
	
	printf("%d\n", Pattern);
	return 0;
}

