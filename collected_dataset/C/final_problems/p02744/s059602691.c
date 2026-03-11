#include<stdio.h>
#include<math.h>
int main()
{
	int n,i,j;
	scanf("%d",&n);
	for(i=0;i<n;i++)
	{
		for(j=0;j<n;j++)
		{
			int x;
			x=(i<j?i:j);
			printf("%c",x+97);
		}
		printf("\n");
	}
}