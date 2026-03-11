#include <stdio.h>

int main(void)
{
	long long a,b,x=1000;
	int i;
	scanf("%lld",&a);
	for(i=0;i<20;i++){
		b=a%1000;
		if(753-b<0){
			b=b-753;
		}else{
			b=753-b;
		}
		if(x>b){
			x=b;
		}
		a=a/10;
//		printf("%d\n",b);
		if(a/100==0){
			break;
		}
	}
	printf("%lld",x);
	return 0;
}
