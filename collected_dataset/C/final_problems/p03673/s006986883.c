#include<stdio.h>
int main()
{
	int n,i,k,a[200000];
	while((scanf("%d",&n)!=EOF)&&(n!=0))
	{
		for(i=0;i<n;i++)
		{
			scanf("%d",&a[i]);
		}
		for(k=0;k<n;k++)
		{	
			//k+1为数组长度 
			int t,j;
			for(j=0;j<(k+1)/2;j++)
			{
				t=a[j];
				a[j]=a[k-j];
				a[k-j]=t;
			}
		}
		for(i=0;i<n-1;i++)
		{
			printf("%d ",a[i]);
		}
		printf("%d",a[n-1]);
	}
	return 0;
}