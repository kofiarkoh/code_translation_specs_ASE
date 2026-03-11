#include <stdio.h>

int main(void)
{
	int  a[5],i,j;
	for(i=0;i<5;i++) scanf("%d",&a[i]);
	for(i=0;i<5;i++)
		for(j=i;j<5;j++)
			if(a[i]<a[j]){
				int t=a[i];
				a[i]=a[j];
				a[j]=t;
			}
	for(i=0;i<5;i++) printf("%d ",a[i]);
	return 0;
}