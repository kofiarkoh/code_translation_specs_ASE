#include<stdio.h>
int main()
{
	int i,a,b,c,d,max=0,s=0;
	scanf("%d",&a);
	while(a!=0){
		s=0;
max=0;
	for(i=0;i<a;i++){
		scanf("%d %d %d",&b,&c,&d);
		c=c+d;
		if(max<c){
			max=c;
			s=b;
		}
	}
	
	printf("%d %d\n",s,max);
	scanf("%d",&a);
	}
	return 0;
}