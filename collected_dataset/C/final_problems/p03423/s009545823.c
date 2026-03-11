#include<stdio.h>
int main()
{
	int i=0,N;
	do
	scanf("%d", &N);
	while (N < 0 || N>1000);
	while (N - 3 != 4 && N - 3 != 5 && N-3>=0)
	{
		N -= 3;
		i++;
	}
	if (N - 3 == 4 || N - 3 == 5)
	{
		i += 2;
	}
	printf("%d", i);
	return 0;
}