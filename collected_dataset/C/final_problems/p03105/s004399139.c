#include <stdio.h>

int main(void){
	int A,B,C;
	int count=0;

	scanf("%d %d %d",&A,&B,&C);

	while(1){
		if(A<=B){
			count++;
			B=B-A;
		}else{
			break;
		}
		if(count==C){
			break;
		}
	}
	printf("%d\n",count);
}