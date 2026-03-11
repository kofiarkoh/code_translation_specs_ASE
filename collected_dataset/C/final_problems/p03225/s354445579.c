#include <stdio.h>
#include <stdlib.h>
#define int long long

int max(int a, int b){
	return a >= b ? a : b;
}

signed main(){
	int H, W, N, i, j, k, ans = 0;
	scanf("%lld%lld", &H, &W);
	N = max(H, W);
	char **s = (char **)malloc(sizeof(char *) * (3 * N + 2));
	for(i = 0; i <= 3 * N + 1; i++){
		s[i] = (char *)malloc(sizeof(char) * (3 * N + 2));
		for(j = 0; j <= 3 * N + 1; j++){
			s[i][j] = '.';
		}
	}
	for(i = N + 1; i <= N + H; i++){
		scanf("%s", &s[i][N + 1]);
		s[i][N + W + 1] = '.';
	}
	int **A = (int **)malloc(sizeof(int *) * (3 * N + 2));
	for(i = 0; i <= 3 * N + 1; i++){
		A[i] = (int *)malloc(sizeof(int) * (3 * N + 2));
		for(j = 0; j <= 3 * N + 1; j++){
			A[i][j] = 0;
		}
	}
	for(i = 1; i <= 3 * N + 1; i++){
		for(j = 1; j <= 3 * N + 1; j++){
			A[i][j] = A[i - 1][j - 1];
			if(s[i][j] == '#'){
				A[i][j]++;
			}
		}
	}
	for(i = N + 1; i <= 2 * N; i++){
		for(j = N + 1; j <= 2 * N; j++){
			if(s[i][j] == '#'){
				for(k = 1; i + k <= 2 * N && j + k <= 2 * N; k++){
					if(s[i + k][j + k] == '#'){
						ans += A[i][j + 2 * k] - A[i - k - 1][j + k - 1];
						ans += A[i + 2 * k][j] - A[i + k - 1][j - k - 1];
					}
				}
			}
		}
	}
	for(i = 0; i <= 3 * N + 1; i++){
		for(j = 0; j <= 3 * N + 1; j++){
			if(j < 3 * N + 1 - j){
				char tmp = s[i][j];
				s[i][j] = s[i][3 * N + 1 - j];
				s[i][3 * N + 1 - j] = tmp;
			}
			A[i][j] = 0;
		}
	}
	for(i = 1; i <= 3 * N + 1; i++){
		for(j = 1; j <= 3 * N + 1; j++){
			A[i][j] = A[i - 1][j - 1];
			if(s[i][j] == '#'){
				A[i][j]++;
			}
		}
	}
	for(i = N + 1; i <= 2 * N; i++){
		for(j = N + 1; j <= 2 * N; j++){
			if(s[i][j] == '#'){
				for(k = 2; i + k <= 2 * N && j + k <= 2 * N; k++){
					if(s[i + k][j + k] == '#'){
						ans += A[i - 1][j + 2 * k - 1] - A[i - k][j + k];
						ans += A[i + 2 * k - 1][j - 1] - A[i + k][j - k];
					}
				}
			}
		}
	}
	printf("%lld\n", ans);
	return 0;
}