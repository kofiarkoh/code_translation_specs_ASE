#include<stdio.h>

int main(void)
{
	int n;
	int i,sum,c,d;
	int j;
	
	while(1)
	{
		scanf("%d",&n);
		if(n==0)break;
		
		c = 0;
		for(i=1;i<=n/2+1;i++){
		  d = 0;
			sum = 0;
			j = i;
			while(sum<n)
			{
				sum += j;
				j++;
				d++;
			}
			if(sum == n && d > 1)c++;
		}
		
		printf("%d\n",c);
	}
	return 0;
}