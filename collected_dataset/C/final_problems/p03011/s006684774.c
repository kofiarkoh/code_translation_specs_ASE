#include <stdio.h>

int main(void){
	int a,sum=0,max=0;
	for(int i=0;i<3;i++){
		scanf("%d",&a);
		if(max<a) max=a;
		sum+=a;
	}
	printf("%d\n",sum-max);
}
