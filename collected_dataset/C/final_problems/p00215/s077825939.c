#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int search(int map[1000][1000],int n,int y,int x,int *hy,int *hx){
	int min = 1999, i, j, sy, sx,flag = 0;
	for(i = 0;i < y;i++){
		for(j = 0;j < x;j++){
			if(map[i][j] == n)
				flag = 1;
		}
	}
	if(flag == 1){
		for(i = 0;i < y;i++){
			for(j = 0;j < x;j++){
				if(map[i][j] == n && abs(*hy - i) + abs(*hx - j) < min){
					sy = i - *hy;
					sx = j - *hx;
					min = abs(sy) + abs(sx);
				}
			}
		}
		*hy = *hy + sy; *hx = *hx + sx;
		return min;
	}
	return 1999;
}
int main(void){
	int i, j, map[1000][1000], y, x, by, bx, gy, gx;
	int *hy, *hx, flag[5],wc[5],min_w,min_i,count,walk;
	char c;
	char str[5];
	hy = (int *)malloc(sizeof(int));
	hx = (int *)malloc(sizeof(int));
	scanf("%d%d",&x,&y);
	while(x != 0 || y != 0){
		for(i = 0;i < y;i++){
			for(j = 0;j < x;j++)
				map[i][j] = -1;
		}
		for(i = 0;i < y;i++){
			for(j = 0;j < x;j++){
				c = '\n';
				while((c = getchar()) == '\n');
				switch(c){
					case '.':
						map[i][j] = 0;
						break;
					case 'S':
						map[i][j] = 6;
						by = i;
						bx = j;
						break;
					case 'G':
						map[i][j] = 7;
						gy = i;
						gx = j;
						break;
					default:
						map[i][j] = atoi(&c);
				}
			}
		}
		for(i = 0;i < 5;i++)
			flag[i] = 0;
		count = 0;
		for(i = 0;i < y;i++){
			for(j = 0;j < x;j++){
				if(1 <= map[i][j] && map[i][j] <= 5 && flag[map[i][j] - 1] == 0){
					count++;
					flag[map[i][j] - 1] = 1;
					if(count > 3)
						break;
				}
			}
			if(count > 3)
				break;
		}
		if(count > 3){
			for(i = 0;i < 5;i++){
				*hy = by; *hx = bx; walk = 0;
				for(j = (i + 1) % 5;i != j;j = (j + 1) % 5)
					walk += search(map,j + 1,y,x,hy,hx);
				wc[i] = walk + abs(*hy - gy) + abs(*hx - gx);
			}
			min_w = wc[0];
			min_i = 1;
			for(i = 1;i < 5;i++){
				if(min_w > wc[i]){
					min_w = wc[i];
					min_i = i + 1;
				}
			}
			printf("%d %d\n",min_i,min_w);
		}
		else
			printf("NA\n");
		for(i = 0;i < 4;i++)
			str[i] = 0;
		while((str[0] = getchar()) == '\n' || str[0] == EOF || str[0] == ' ');
		for(i = 1;i < 4 && (c = getchar()) != ' ';i++)
			str[i] = c;
		x = atoi(str);	 
		scanf("%d",&y);
	}
	return 0;
}