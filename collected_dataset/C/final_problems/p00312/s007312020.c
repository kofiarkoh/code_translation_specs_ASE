#include <stdio.h>
int main(void){
	int D,L,p,o;
	scanf("%d %d",&D,&L);
	p=D/L;
	o=D%L;
	if(o==0)
	printf("%d\n",p);
	else
	printf("%d\n",o+p);
	return 0;
}
