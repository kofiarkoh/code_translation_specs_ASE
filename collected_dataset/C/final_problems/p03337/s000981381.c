#include <stdio.h>

int main()
{
	int A, B, maior;

	scanf("%d %d", &A, &B);

	if ((A + B) > (A - B) && (A + B) > (A * B))
	{
		maior = (A + B);
	}
	else if ((A - B) > (A + B) && (A - B) > (A * B))
	{
		maior = (A - B);
	}
	else if ((A * B) > (A + B) && (A * B) > (A - B))
	{
		maior = (A * B);
	}
	else if ((A + B) == (A - B) && (A + B) == (A * B))
	{
		maior = (A + B);
	}

	printf("%d", maior);
	return 0;
}
