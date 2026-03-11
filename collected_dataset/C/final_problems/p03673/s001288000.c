#include<stdio.h>
int main()
{
	int n,i,j;
	scanf("%d",&n);
	int a[n],t[n];
	for(i=0;i<n;i++)
        {
    	scanf("%d",&a[i]);
    	for(j=0;j<i/2+1;j++)
    	{
    		t[j]=a[j];
    		a[j]=a[i-j];
    	}
    	for(j=i/2+1;j<i+1;j++)
    		a[j]=t[i-j];
	}
	for(i=0;i<n;i++)
	    printf("%d ",a[i]);
}