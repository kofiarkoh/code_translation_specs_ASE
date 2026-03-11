#define _CRT_SECURE_NO_WARNINGS
#define _USE_MATH_DEFINES
#include<stdio.h>
#include<string.h>
int main() {
	char bw[10], *pt;
	int board[3][3], i, j, k, flg, m;
	while (~fscanf(stdin, "%s", bw)) {
		getchar();
		pt = bw;
		flg = j = m = k = 0;
		memset(board, '\0', sizeof(board));
		while (*pt) {
			if (*pt == 'o') board[j/3][k++%3] = 1;
			else if (*pt == 'x') board[j/3][k++%3] = 2;
			else board[j/3][k++] = 0;
			pt++;
			j++;
		}
		for (k = 0; k < 3; k++) {
			if (board[k][0] == board[k][1] && board[k][0] == board[k][2]) { m = board[k][0]; flg = 1; break; }
			if (board[0][k] == board[1][k] && board[0][k] == board[2][k]) { m = board[0][k]; flg = 1; break; }
			if (board[1][1] == board[0][0] && board[1][1] == board[2][2]) { m = board[1][1]; flg = 1; break; }
			if (board[1][1] == board[0][2] && board[1][1] == board[2][0]) { m = board[1][1]; flg = 1; break; }
		}
		if (m == 0) flg = 0;
		if (!flg) fprintf(stdout, "d\n");
		else fprintf(stdout, "%c\n", m == 1 ? 'o' : 'x');
		memset(bw, '\0', sizeof(bw));
	}
	return 0;
}