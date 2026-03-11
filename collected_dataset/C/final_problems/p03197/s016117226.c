#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, A, i, f = 0;
	scanf("%d", &N);
	for(i = 0; i < N; i++){
		scanf("%d", &A);
		if(A % 2 == 1){
			f = 1;
		}
	}
	if(f == 0){
		printf("second\n");
	}
	else{
		printf("first\n");
	}
	return 0;
}