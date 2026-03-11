#include <stdio.h>

char S[80001], A[80000], B[80001][11];

int char_to_int(char c){
	return (int)c - (int)'0';
}

int main(){
	int N, i, j, ans;
	while(1){
		scanf("%s", S);
		for(N = 0; S[N] != '\0'; N++){}
//		printf("N = %d\n", N);
		if(N == 1 && S[0] == '0'){
			return 0;
		}
		A[0] = char_to_int(S[0]);
		for(i = 1; i < N; i++){
//			printf("i = %d\n", i);
			if(i % 2 == 0){
//				printf("cti = %d\n", char_to_int(S[i]));
				A[i] = (A[i - 1] + char_to_int(S[i])) % 11;
			}
			else{
				A[i] = (A[i - 1] - char_to_int(S[i]) + 11) % 11;
			}
		}
/*		printf("A:\n");
		for(i = 0; i < N; i++){
			printf("%d ", A[i]);
		}
		printf("\n");
*/		for(j = 0; j < 11; j++){
			B[N][j] = 0;
		}
		for(i = N - 1; i >= 0; i--){
			for(j = 0; j < 11; j++){
				B[i][j] = B[i + 1][j] + (j == A[i] ? 1 : 0);
			}
		}
		ans = B[0][0];
		for(i = 0; i < N - 1; i++){
			if(S[i + 1] != '0'){
//				printf("i = %d\n", i);
				ans += B[i + 1][A[i]];
//				printf("ans = %d\n", ans);
			}
		}
		printf("%d\n", ans);
	}
}
