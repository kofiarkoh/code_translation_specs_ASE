#include <stdio.h>
#include <string.h>

int main(void){
	char S[100],T[100];
	scanf("%s",&S);
	scanf("%s",&T);
	int size = strlen(S);//strlen()で文字列の長さ
	char U;
	int i,j;
	for(i=0;i<size;i++){
		for(j=0;j<size-1;j++){
			U = S[j];
			S[j] = S[j+1];
			S[j+1] = U;
		}
		if(strcmp(S,T) == 0){
			printf("Yes");
			return 0;
		}
	}
	printf("No");
	return 0;
}