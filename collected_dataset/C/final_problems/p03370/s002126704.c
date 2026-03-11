#include<stdio.h>
int main(void)
{
	int i,n,x,array[100],need=0;
	scanf("%d",&n);
	scanf("%d",&x);
	for(i=0;i<n;i++)
	{
		scanf("%d",&array[i]);
		need=need+array[i];
	}
	x=x-need;
	for(i=0;i<n;i++)
	{
		if(array[0]>array[i]) array[0]=array[i];
	}
	int count=n,f=0;
	while(f==0)
	{if(x-array[0]>0) count++;
	if(x-array[0]>0) x=x-array[0];
        if(x-array[0]==0) count++;
	 if(x-array[0]==0) f=1;
	 if(x-array[0]<0) f=1;
	}
	printf("%d",count);
	return 0;
}
