#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(){
	int L, i, j, k, l, A, B, tmp;
	char c;
	scanf("%d", &L);
	char *S = (char *)malloc(sizeof(char) * (L + 1));
	char *T = (char *)malloc(sizeof(char) * (L + 1));
	char *strtmp;
	scanf("%s", S);
	scanf("%s", T);
	for(A = 0; S[A] != '\0'; A++){}
	for(B = 0; T[B] != '\0'; B++){}
	if(A > B){
		strtmp = S;
		S = T;
		T = strtmp;
		tmp = A;
		A = B;
		B = tmp;
	}
//	printf("S:%s\n", S);
//	printf("T:%s\n", T);
	c = T[A];
	T[A] = '\0';
//	printf("strcmp(S, T) = %d\n", strcmp(S, T));
//	printf("c = %c\n", c);
	if(strcmp(S, T) == 0 && c < S[0]){
//		printf("test\n");
		T[A] = c;
		strtmp = S;
		S = T;
		T = strtmp;
		tmp = A;
		A = B;
		B = tmp;
	}
	else{
		T[A] = c;
		if(strcmp(S, T) > 0){
			strtmp = S;
			S = T;
			T = strtmp;
			tmp = A;
			A = B;
			B = tmp;
		}
	}
	for(l = 0; B * l <= L; l++){
		if((L - B * l) % A == 0){
			k = (L - B * l) / A;
			break;
		}
	}
	for(i = 0; i < k; i++){
		for(j = 0; j < A; j++){
			printf("%c", S[j]);
		}
	}
	for(i = 0; i < l; i++){
		for(j = 0; j < B; j++){
			printf("%c", T[j]);
		}
	}
	printf("\n");
	return 0;
}