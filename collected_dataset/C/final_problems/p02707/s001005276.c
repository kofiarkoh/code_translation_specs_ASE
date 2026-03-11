#include <stdio.h>

int main()
{
	int N;
	if(scanf("%d",&N)<1)
		return -1;

	int A[N];
	int B[N];
	for(int i=1;i<N;i++)
	{
		if(scanf("%d",&A[i])<1)
			return -1;
		B[i]=0;
	}

	for(int i=1;i<N;i++)
	{
		B[A[i]]++;
	}

	for(int i=1;i<N;i++)
	{
		printf("%d\n",B[i]);
	}
	printf("0\n");
	return 0;
}