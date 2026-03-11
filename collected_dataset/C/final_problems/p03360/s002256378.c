#include<stdio.h>
int main()
{
	int a,b,c,k,max,x,y,z;
	scanf("%d %d %d %d",&a,&b,&c,&k);
	x=(a*2)*k+b+c;
	y=(b*2)*k+a+c;
	z=(c*2)*k+a+b;
	max=-1;
	if(max<x){
		max=x;
	}
	if(max<y){
		max=y;
	}
	if(max<z){
		max=z;
	}
	printf("%d\n",max);
	return 0;
}