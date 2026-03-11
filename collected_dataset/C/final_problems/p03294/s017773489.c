#include<stdio.h>

int main()
{
	int N = 0;
	scanf("%d", &N);
	long sum = 0;
	for(int i = 0; i < N; i++)
	{
		int j = 0;
		scanf("%d", &j);
		sum += j - 1;
	}
	printf("%ld\n", sum);
	return 0;
}