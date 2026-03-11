#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, i, j, k, l, wnum = 0;
	scanf("%d", &N);
	char **A = (char **)malloc(sizeof(char *) * (N + 2));
	A[0] = (char *)malloc(sizeof(char) * (N + 2));
	for(i = 1; i <= N; i++){
		A[i] = (char *)malloc(sizeof(char) * (N + 2));
		scanf("%s", &A[i][1]);
	}
	A[N + 1] = (char *)malloc(sizeof(char) * (N + 2));
	for(i = 0; i <= N + 1; i++){
		A[i][0] = 'x';
		A[i][N + 1] = 'x';
		A[0][i] = 'x';
		A[N + 1][i] = 'x';
	}
/*	printf("test\n");
	for(i = 0; i <= N + 1; i++){
		for(j = 0; j <= N + 1; j++){
			printf("%c", A[i][j]);
		}
		printf("\n");
	}
*/	for(i = 1; i <= N; i++){
		for(j = 1; j <= N; j++){
			if(A[i][j] == 'o'){
				for(k = i; A[k][j] == 'o'; k++){
					for(l = j; A[k][l] == 'o'; l++){
						A[k][l] = 'x';
					}
				}
				wnum++;
			}
		}
	}
	printf("%d\n", (wnum - 1) / 3);
	return 0;
}