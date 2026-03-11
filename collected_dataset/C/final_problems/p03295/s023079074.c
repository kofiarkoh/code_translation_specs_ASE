#include<stdio.h>
int main()
{
	int n,m,i,a[100001],b[100001],min,count=0;
	scanf("%d %d\n%d %d",&n,&m,&a[1],&b[1]);
	min = b[1];
	for(i=2;i<=m;i++){
		scanf("%d %d\n",&a[i],&b[i]);
		if(b[i]<min)min=b[i];
	}
	for(i=1;i<=m;i++){
		if(a[i]>min)count++;
	}
	printf("%d",count+1);
	return 0;
}