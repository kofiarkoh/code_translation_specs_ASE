#include<stdio.h>

int main(void)
{
	int i, n, t[100];
	
	i=-1;
	while(scanf("%d",&n)!=EOF)
	{
		if(n==0)
		{
			printf("%d\n",t[i]);
			i--;
		}
		else
		{
			i++;
			t[i]=n;
		}
	}
	return 0;
}