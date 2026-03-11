#include <stdio.h>

int main(void)
{
	int N;
	scanf("%d",&N);
	
	int L[2*N];
	int i,j,tmp;
	for(i=0;i<2*N;i++)
	{
		scanf("%d",&L[i]);
	}
	for(i = 0 ; i < 2*N;i++)
	{
		for(j=i;j < 2*N-1;j++)
		{
			if(L[j] > L[j+1])
			{
				tmp = L[j];
				L[j] = L[j+1];
				L[j+1] = tmp;
			}
		}
	}
	int ans = 0;
	for(i = 0; i < 2*N;i+=2)
	{
		ans = ans+L[i];
	}
	printf("%d",ans);
	return 0;
}
