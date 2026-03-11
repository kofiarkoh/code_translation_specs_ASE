#include <stdio.h>

int main(){
	int D;
	int a;

	scanf("%d",&D);
	if (D == 25) printf("Christmas\n");
	if (22 <= D && D <= 24){
		printf("Christmas ");
		a = 25 - D;
		for( int i = 0; i < a; i++){
			printf("Eve ");
		}
		printf("\n");
	}
	return 0;
}