#include<stdlib.h>
#include<stdio.h>
#include<string.h>
#define TRUE 1
#define WHITE 0
#define BLACK 1

int maxlength();

int main(void) {
	int i;
	char black[6] = "Black";
	char white[6] = "White";
	char draw[6] = "Draw";
	char winner[6] = "\0";
	int turn; 
	int n; 
	int m; 
	int p; 
	int field[8][8][8]; 
	int bmax; 
	int wmax; 
	int max;
	int x,y;
	int instant;
	
	while(TRUE) {
		scanf("%d %d %d", &n, &m, &p);
		if(n == 0 && m == 0 && p == 0) break;
		
		for(i=0;i<8;i++) {
			for(int j=0;j<8;j++) {
				for(int k=0;k<8;k++) {
					field[i][j][k] = -1;
				}
			}
		}
		bmax = 0;
		wmax = 0;
		max = 0;
		for(turn = 1;turn <= p; turn++) {
			scanf("%d %d", &x, &y);
			
			for(i=0;i<8;i++) {
				if(field[x][y][i] == -1) {
					field[x][y][i] = turn % 2;
					break;
				}
			}
			instant = maxlength(field, turn, x, y, i, n);
			if(turn % 2 == BLACK && instant > bmax) {
				bmax = instant;
			}
			if(turn % 2 == WHITE && instant > wmax) {
				wmax = instant;
			}
			if(max < instant) max = instant;
			if(max == m) {
				for(int j=turn+1;j <=p;j++) scanf("%d %d", &x, &y);
				break;
			}
			if(turn == p) break;
		}
		if(bmax > wmax) strcpy(winner, black);
		else if(wmax > bmax) strcpy(winner, white);
		else strcpy(winner, draw);
		
		printf("%s", winner);
		if(strcmp(draw,winner) != 0) printf(" %d", turn);
		printf("\n");
	}
	
	exit(0);
}

int maxlength(int field[][8][8],int turn,int x,int y,int z,int n) {
	int max=0;
	int instant[9];
	int out[18];
	int sum=0;
	
	for(int i=0;i<9;i++) {
		instant[i] = 1;
	}
	for(int i=0;i<18;i++) out[i] = 0;
	
	for(int i=1;;i++) {
		if(y+i <= n && z+i <= n && field[x][y+i][z+i] == turn % 2) instant[0]++;
		else out[0] = -1;
		if(y-i >= 1 && z-i >= 1 && field[x][y-i][z-i] == turn % 2) instant[0]++;
		else out[1] = -1;
		
		if(y+i <= n && z-i >= 1 && field[x][y+i][z-i] == turn % 2) instant[1]++;
		else out[2] = -1;
		if(y-i >= 1 && z+i <= n && field[x][y-i][z+i] == turn % 2) instant[1]++;
		else out[3] = -1;
		if(x+i <= n && z+i <= n && field[x+i][y][z+i] == turn % 2) instant[2]++;
		else out[4] = -1;
		if(x-i >= 1 && z-i >= 1 && field[x-i][y][z-i] == turn % 2) instant[2]++;
		else out[5] = -1;
		
		if(x+i <= n && z-i >= 1 && field[x+i][y][z-i] == turn % 2) instant[3]++;
		else out[6] = -1;
		if(x-i >= 1 && z+i <= n && field[x-i][y][z+i] == turn % 2) instant[3]++;
		else out[7] = -1;
		if(y+i <= n && x+i <= n && field[x+i][y+i][z] == turn % 2) instant[4]++;
		else out[8] = -1;
		if(y-i >= 1 && x-i >= 1 && field[x-i][y-i][z] == turn % 2) instant[4]++;
		else out[9] = -1;
		
		if(y+i <= n && x-i >= 1 && field[x-i][y+i][z] == turn % 2) instant[5]++;
		else out[10] = -1;
		if(y-i >= 1 && x+i <= n && field[x+i][y-i][z] == turn % 2) instant[5]++;
		else out[11] = -1;
		if(z+i <= n && field[x][y][z+i] == turn % 2) instant[6]++;
		else out[12] = -1;
		
		if(z-i >= 1 && field[x][y][z-i] == turn % 2) instant[6]++;
		else out[13] = -1;
		if(y+i <= n && field[x][y+i][z] == turn % 2) instant[7]++;
		else out[14] = -1;
		
		if(y-i >= 1 && field[x][y-i][z] == turn % 2) instant[7]++;
		else out[15] = -1;
		if(x+i <= n && field[x+i][y][z] == turn % 2) instant[8]++;
		else out[16] = -1;
		
		if(x-i >= 1 && field[x-i][y][z] == turn % 2) instant[8]++;
		else out[17] = -1;
		
		sum = 0;
		for(int j=0;j<18;j++) {
			sum+=out[j];
		}
		if(sum <= -18) break;
	}
	
	for(int i=0;i<9;i++) {
		if(max < instant[i]) max = instant[i];
	}
	return max;
}