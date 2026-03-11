#include<stdio.h>
int main()
{
	    int a,b,num[200005];
	    scanf("%d %d",&a,&b);
		int num1=0,sum=0,i,j,n;
		for(i=0;i<a;i++)
		scanf("%d",&num[i]);
		for(i=0;i<a;i++)
		{
			sum=0;
			for(j=i;j<a;j++)
			{
				sum+=num[j];
			    if(sum/(j-i+1)>=b) num1++;
	        }
		}
		printf("%d\n",num1);
 } 