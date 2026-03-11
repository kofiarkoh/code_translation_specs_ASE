#include <stdio.h>

int main(void){
	int c,r;
	while(scanf("%d %d",&c,&r) != EOF){
		if((c==0)&&(r==0)){
			return 0;
		}
		if((c*r)%2){
			printf("no\n");
		}
		else{
			printf("yes\n");
		}
	}
}