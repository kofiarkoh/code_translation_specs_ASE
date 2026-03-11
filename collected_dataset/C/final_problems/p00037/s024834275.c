#include <stdio.h>
#define LEFT 0
#define RIGHT 1
#define UP 2
#define DOWN 3

int main(void){
	int wall_s[5][4];
	int wall_l[4][5];
	int x, y;
	int dir;
	int i, j;

	//üÍ
	for(j=0; j<4; j++){
		for(i=0; i<4; i++){
			scanf("%1d", &wall_s[j][i]);
		}
		for(i=0; i<5; i++){
			scanf("%1d", &wall_l[j][i]);
		}
	}
	for(i=0; i<4; i++){
		scanf("%1d", &wall_s[4][i]);
	}

	x = 0;
	y = 0;	
	dir = RIGHT;
	while(1){
		if(dir == RIGHT){
			if(y-1 >= 0 && wall_l[y-1][x] == 1){
				y--;
			   	printf("U");
				dir = UP;
			}else if(x+1 <= 4 && wall_s[y][x] == 1){
				x++;
				printf("R");
				dir = RIGHT;
			}else if(y+1 <= 4 && wall_l[y][x] == 1){
				y++;
				printf("D");
				dir = DOWN;
			}else if(x-1 >= 0 && wall_s[y][x-1] == 1){
				x--;
				printf("L");
				dir = LEFT;
			}
		}else if(dir == LEFT){
			if(y+1 <= 4 && wall_l[y][x] == 1){
				y++;
			   	printf("D");
				dir = DOWN;
			}else if(x-1 >= 0 && wall_s[y][x-1] == 1){
				x--;
				printf("L");
				dir = LEFT;
			}else if(y-1 >= 0 && wall_l[y-1][x] == 1){
				y--;
				printf("U");
				dir = UP;
			}else if(x+1 <= 4 && wall_s[y][x] == 1){
				x++;
				printf("R");
				dir = RIGHT;
			}
		}else if(dir == UP){
			if(x-1 >= 0 && wall_s[y][x-1] == 1){
				x--;
				printf("L");
				dir = LEFT;
			}else if(y-1 >= 0 && wall_l[y-1][x] == 1){
				y--;
				printf("U");
				dir = UP;
			}else if(x+1 <= 4 && wall_s[y][x] == 1){
				x++;
				printf("R");
				dir = RIGHT;
			}else if(y+1 <= 4 && wall_l[y][x] == 1){
				y++;
			   	printf("D");
				dir = DOWN;
			}
		}else if(dir == DOWN){
			if(x+1 <= 4 && wall_s[y][x] == 1){
				x++;
				printf("R");
				dir = RIGHT;
			}else if(y+1 <= 4 && wall_l[y][x] == 1){
				y++;
			   	printf("D");
				dir = DOWN;
			}else if(x-1 >= 0 && wall_s[y][x-1] == 1){
				x--;
				printf("L");
				dir = LEFT;
			}else if(y-1 >= 0 && wall_l[y][y-1] == 1){
				y--;
				printf("U");
				dir = UP;
			}
		}
		if(x == 0 && y == 0) break;
		//printf("%d %d\n", x, y); fflush(stdout);
	}
	puts("");

	return 0;
}