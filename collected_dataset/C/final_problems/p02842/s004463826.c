#include<stdio.h>
int main(void)
{
	int n,x,i;
	scanf("%d", &n);
	for(i=1; i<n; i++)
	{
		x=i*1.08;
		if(x==n)
		{
			printf("%d\n", i);
			break;
		}
		if(i==n-1)
		printf(":(\n");
	}
	
	return 0;
}