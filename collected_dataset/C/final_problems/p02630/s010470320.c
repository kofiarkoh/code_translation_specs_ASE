#include<stdio.h>
int main()
{
	int n,i,sum=0;
	scanf("%d",&n);
	int arr[n];
	for(i=0;i<n;i++)
	{
		scanf("%d",&arr[i]);
	}
	int q;
	scanf("%d",&q);
	while(q--)
	{
		int b,c,sum=0;
		scanf("%d %d",&b,&c);
		for(i=0;i<n;i++)
		{
			if(arr[i]==b)
			arr[i]=c;
			sum+=arr[i];
		}
		printf("%d\n",sum);	
	}	
	
}