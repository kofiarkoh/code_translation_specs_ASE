#include <stdio.h>
#include <stdlib.h>
#define max_size (int)(1e5 + 1)

int max(int a, int b){
	return a >= b ? a : b;
}

int min(int a, int b){
	return a <= b ? a : b;
}

int main(){
	int i, a_num = 0, b_num = 0, c_num = 0;
	char *S = (char *)malloc(sizeof(char) * max_size);
	scanf("%s", S);
	for(i = 0; S[i] != '\0'; i++){
		if(S[i] == 'a'){
			a_num++;
		}
		if(S[i] == 'b'){
			b_num++;
		}
		if(S[i] == 'c'){
			c_num++;
		}
	}
	if(max(a_num, max(b_num, c_num)) - min(a_num, min(b_num, c_num)) <= 1){
		printf("YES\n");
	}
	else{
		printf("NO\n");
	}
	return 0;
}