#include<stdio.h>

int w, h;
int map[50][50];

void visit(int x, int y){
	int i, j;
	if (x < 0 || x >= w || y < 0 || y >= h) return;
	if (map[y][x] == 0) return;
	map[y][x] = 0;
	for (i = -1; i <=1; i++){
		for (j = -1; j <=1; j++){
			visit(x + i, y + j);
		}
	}
}

int main(void){
	int i, j;
	int ans;
	while (scanf("%d %d", &w, &h), w != 0 || h != 0){
		for (i = 0; i < h; i++){
			for (j = 0; j < w; j++){
				scanf("%d", &map[i][j]);
			}
		}
		ans = 0;
		for (i = 0; i < h; i++){
			for (j = 0; j < w; j++){
				if (map[i][j]){
					ans++;
					visit(j, i);
				}
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}