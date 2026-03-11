#include<stdio.h>
int main()
{
	int N,b[4],count,j,i;
	scanf("%d",&N);
	for(i=0;i<4;i++)
	{
		b[i]=N%10;
		N = N/10;
	}
	for(i=0;i<4;i++)
	{
		count = 1;
		for(j=(i+1);j<4;j++)
		{
			if(b[i]==b[j])
			{
				count++;
			}	
		}
		if(count==3) break;
	}
	if(count==3) printf("Yes\n");
	else printf("No\n");
	return 0;
}