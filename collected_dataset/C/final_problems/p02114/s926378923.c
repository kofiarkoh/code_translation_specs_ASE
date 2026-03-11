#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, i, j, k, l, ans = 0;
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
	for(i = 2; i <= N; i += 2){
		for(j = 2; j <= N; j += 2){
//			printf("(i, j) = (%d, %d)\n", i, j);
			if(A[i][j] == 'x' && A[i - 1][j] == 'x' && A[i + 1][j] == 'x' && A[i][j - 1] == 'x' && A[i][j + 1] == 'x'){
				for(k = i + 1; ; k++){
					if(A[k][j] == 'o'){
						break;
					}
					if(A[k][j - 1] == 'x' && A[k][j] == 'x' && A[k][j + 1] == 'x'){
						A[k][j] = 'o';
						break;
					}
				}
				for(l = j + 1; ; l++){
					if(A[i][l] == 'o'){
						break;
					}
					if(A[i - 1][l] == 'x' && A[i][l] == 'x' && A[i + 1][l] == 'x'){
						A[i][l] = 'o';
						break;
					}
				}
				ans++;
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}