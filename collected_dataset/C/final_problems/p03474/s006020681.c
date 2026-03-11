#include <stdio.h>
#include <string.h>

int main() {
	int A,B;
	char code[20];
	int aaa, counts = 0;
	scanf("%d %d", &A, &B);
	scanf("%s", code);
	aaa = strlen(code);
	
	if(!(code[A] == '-')){
		printf("No");
		return 0;
	}
	for(int i=0;i<aaa;i++){
		if(code[i] == '-'){
			counts++;
		}
	}
	if(counts > 1){
		printf("No");
		return 0;
	}
	
	printf("Yes");
			
return 0;
}