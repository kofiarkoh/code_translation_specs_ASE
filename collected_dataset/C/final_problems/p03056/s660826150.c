#include <stdio.h>
#include <stdlib.h>

int max(int a, int b){
	return a >= b ? a : b;
}

int min(int a, int b){
	return a <= b ? a : b;
}

int **B;

int BorW(char c){
	if(c == '#'){
		return 1;
	}
	else{
		return 0;
	}
}

int _B(int x, int y){
	if(x < 0 || y < 0){
		return 0;
	}
	else{
		return B[x][y];
	}
}

int sum(int x1, int x2, int y1, int y2){
	return _B(x2 - 1, y2 - 1) - _B(x2 - 1, y1 - 1) - _B(x1 - 1, y2 - 1) + _B(x1 - 1, y1 - 1);
}

int main(){
	int H, W, i, j;
	scanf("%d%d", &H, &W);
	char **A = (char **)malloc(sizeof(char *) * H);
	for(i = 0; i < H; i++){
		A[i] = (char *)malloc(sizeof(char) * (W + 1));
		scanf("%s", A[i]);
	}
	B = (int **)malloc(sizeof(int *) * H);
	for(i = 0; i < H; i++){
		B[i] = (int *)malloc(sizeof(int) * W);
	}
	B[0][0] = BorW(A[0][0]);
	for(i = 1; i < H; i++){
		B[i][0] = B[i - 1][0] + BorW(A[i][0]);
	}
	for(j = 1; j < W; j++){
		B[0][j] = B[0][j - 1] + BorW(A[0][j]);
	}
	for(i = 1; i < H; i++){
		for(j = 1; j < W; j++){
			B[i][j] = B[i - 1][j] + B[i][j - 1] - B[i - 1][j - 1] + BorW(A[i][j]);
		}
	}

	int M = 17;
	int c, x, y, x1, x2, y1, y2;
	int ****dp1 = (int ****)malloc(sizeof(int ***) * M);
	for(c = 0; c < M; c++){
		dp1[c] = (int ***)malloc(sizeof(int **) * H);
		for(x1 = 0; x1 < H; x1++){
			dp1[c][x1] = (int **)malloc(sizeof(int *) * (H + 1));
			for(x2 = x1 + 1; x2 <= H; x2++){
				dp1[c][x1][x2] = (int *)malloc(sizeof(int) * (W + 1));
				for(y = 0; y <= W; y++){
					dp1[c][x1][x2][y] = W;
				}
			}
		}
	}
	int ****dp2 = (int ****)malloc(sizeof(int ***) * M);
	for(c = 0; c < M; c++){
		dp2[c] = (int ***)malloc(sizeof(int **) * W);
		for(y1 = 0; y1 < W; y1++){
			dp2[c][y1] = (int **)malloc(sizeof(int *) * (W + 1));
			for(y2 = y1 + 1; y2 <= W; y2++){
				dp2[c][y1][y2] = (int *)malloc(sizeof(int) * (H + 1));
				for(x = 0; x <= H; x++){
					dp2[c][y1][y2][x] = H;
				}
			}
		}
	}
//	printf("test3\n");

	for(x1 = 0; x1 < H; x1++){
		for(x2 = x1 + 1; x2 <= H; x2++){
//			printf("(x1, x2) = (%d, %d)\n", x1, x2);
			for(y1 = 0, y2 = 0; y1 < W; y1++){
//				printf("y1 = %d\n", y1);
				while(y2 < W){
					int s = sum(x1, x2, y1, y2 + 1);
//					printf("(y2, s) = (%d, %d)\n", y2, s);
					if(0 < s && s < (x2 - x1) * (y2 + 1 - y1)){
						break;
					}
					y2++;
				}
				dp1[0][x1][x2][y1] = y2;
//				printf("dp1[%d][%d][%d][%d] = %d\n", 0, x1, x2, y1, dp1[0][x1][x2][y1]);
			}
		}
	}
	for(y1 = 0; y1 < W; y1++){
		for(y2 = y1 + 1; y2 <= W; y2++){
			for(x1 = 0, x2 = 0; x1 < H; x1++){
				while(x2 < H){
					int s = sum(x1, x2 + 1, y1, y2);
					if(0 < s && s < (x2 + 1 - x1) * (y2 - y1)){
						break;
					}
					x2++;
				}
				dp2[0][y1][y2][x1] = x2;
//				printf("dp2[%d][%d][%d][%d] = %d\n", 0, y1, y2, x1, dp2[0][y1][y2][x1]);
			}
		}
	}
//	printf("test4\n");

	for(c = 1; c < M; c++){
//		printf("c = %d\n", c);
		for(x1 = 0; x1 < H; x1++){
			for(x2 = x1 + 1; x2 <= H; x2++){
//				printf("(x1, x2) = (%d, %d)\n", x1, x2);
				for(y1 = 0, y2 = 0; y1 < W; y1++){
					y2 = max(y2, y1);
//					printf("y1 = %d\n", y1);
					while(y2 < W){
//						printf("y2 = %d\n", y2);
						if(dp2[c - 1][y1][y2 + 1][dp2[c - 1][y1][y2 + 1][x1]] < x2){
							break;
						}
						y2++;
					}
					dp1[c][x1][x2][y1] = max(y2, dp1[c - 1][x1][x2][dp1[c - 1][x1][x2][y1]]);
//					printf("dp1[%d][%d][%d][%d] = %d\n", c, x1, x2, y1, dp1[c][x1][x2][y1]);
				}
			}
		}
//		printf("test4.5\n");
		for(y1 = 0; y1 < W; y1++){
			for(y2 = y1 + 1; y2 <= W; y2++){
				for(x1 = 0, x2 = 0; x1 < H; x1++){
					x2 = max(x2, x1);
					while(x2 < H){
						if(dp1[c - 1][x1][x2 + 1][dp1[c - 1][x1][x2 + 1][y1]] < y2){
							break;
						}
						x2++;
					}
					dp2[c][y1][y2][x1] = max(x2, dp2[c - 1][y1][y2][dp2[c - 1][y1][y2][x1]]);
//					printf("dp2[%d][%d][%d][%d] = %d\n", c, y1, y2, x1, dp2[c][y1][y2][x1]);
				}
			}
		}
	}
	int ans = M;
	for(c = 0; c < M; c++){
		if(dp1[c][0][H][0] >= W){
			ans = min(ans, c);
		}
		if(dp2[c][0][W][0] >= H){
			ans = min(ans, c);
		}
	}
	printf("%d\n", ans);
	return 0;
}