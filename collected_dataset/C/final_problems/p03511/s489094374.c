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
	c = T[B];
	T[B] = '\0';
	if(strcmp(S, T) == 0 && c < S[0]){
		T[B] = c;
		strtmp = S;
		S = T;
		T = strtmp;
		tmp = A;
		A = B;
		B = tmp;
	}
	else{
		T[B] = c;
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