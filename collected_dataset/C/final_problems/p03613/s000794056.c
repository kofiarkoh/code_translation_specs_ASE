#include<stdio.h>
#include<math.h>
int a[10005];	
int b[10005];
int main()
{
	int a[10005];	
int b[10005];
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++)
	scanf("%d",&a[i]);
  int t=0;
	for(int i=0;i<n;i++)
	{	int sum=0;
		for(int j=0;j<n;j++)
		{
			if(fabs(a[j]-a[i])==1||a[j]-a[i]==0)
			{sum++;
			}
			b[t++]=sum;
		}
	}
	int max=b[0];
	for(int i=0;i<t;i++)
	if(b[i]>max)
	max=b[i];
	printf("%d\n",max);
}