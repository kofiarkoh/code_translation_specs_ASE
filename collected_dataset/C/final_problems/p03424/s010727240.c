#include <stdio.h>
#include <stdlib.h>

int main(void){
	int N,i;
	char* s;
	scanf("%d",&N);
	s = (char*)malloc(sizeof(char)*N);
	for(i=0;i<N;i++) scanf(" %c",&s[i]);
	for(i=0;i<N;i++) {
		if(s[i]=='Y'){
			printf("Four\n");
			return 0;
		}
	}
	printf("Three\n");
	return 0;
}