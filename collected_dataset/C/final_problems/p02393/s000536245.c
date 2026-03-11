#include <stdio.h>
int main(void){
	int a,b,c;
	int kae;

	scanf("%d",&a);
	scanf("%d",&b);
	scanf("%d",&c);
	
	if(a>b){
		kae=a;
		a=b;
		b=kae;
	}
	
	if(b>c){
		kae=b;
		b=c;
		c=kae;
	}
	
	if(a>b){
		kae=a;
		a=b;
		b=kae;
	}
	
	printf("%d %d %d\n",a,b,c);
	
	return 0;
	
}