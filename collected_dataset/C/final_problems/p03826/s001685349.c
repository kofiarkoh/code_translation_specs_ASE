#include<stdio.h>


int main(int argc, char const *argv[])
{
	int A,B,C,D,area1,area2;
	scanf("%d %d %d %d",&A,&B,&C,&D);
	area1=1;
	area2=1;
	area1=A*B;
	area2=C*D;
	printf("%d",area1>area2?area1:area2 );
	return 0;
}