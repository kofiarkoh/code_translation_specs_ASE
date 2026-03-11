#include<stdio.h>
int main(void)
{
	int i,w,n;
	scanf("%d",&w);
	n=0;
	for(i=1;i<=w;i++){
		n=n+32;
	}
	printf("%d\n",n);
	return 0;
}
