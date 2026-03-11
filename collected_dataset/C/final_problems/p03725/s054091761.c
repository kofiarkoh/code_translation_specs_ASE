#include <stdio.h>

int H, W, K;
char room[800][800];
int visited[800][800];

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

int check_finish(void) {
	int i;
	for (i = 0; i < H; i++) 
		if (room[i][0] == 'S' || room[i][W - 1] == 'S')
			return 1;
	for (i = 0; i < W; i++)
		if (room[0][i] == 'S' || room[H - 1][i] == 'S')
			return 1;
	return 0;
}

void clear_visited() {
	int i, j;
	for (i = 0; i < H; i++)
		for (j = 0; j < W; j++)
			visited[i][j] = 0;
}

void visit(int h, int w) {
	if (!visited[h][w] && room[h][w] != '#' && 0 <= h && h <= H - 1 && 0 <= w && w <= W - 1) {
		visited[h][w] = 2;
		visit(h - 1, w);
		visit(h + 1, w);
		visit(h, w - 1);
		visit(h, w + 1);		
	}
}

void replace(int h, int w, char c1, char c2, int range) {
	if (!visited[h][w] && range >= 0 && 0 <= h && h <= H - 1 && 0 <= w && w <= W - 1) {
		visited[h][w] = 1;
		if (room[h][w] == c1) {
			// printf("replace(%d, %d)\n", h, w);
			room[h][w] = c2;
			replace(h - 1, w, c1, c2, range - 1);
			replace(h + 1, w, c1, c2, range - 1);
			replace(h, w - 1, c1, c2, range - 1);
			replace(h, w + 1, c1, c2, range - 1);
		}
	}
}

int not_need(int h, int w) {
	char c = room[h][w];
	if (h - 1 >= 0 && room[h - 1][w] != c)
		return 0;
	if (h + 1 <= H - 1 && room[h + 1][w] != c)
		return 0;
	if (w - 1 >= 0 && room[h][w - 1] != c)
		return 0;
	if (w + 1 <= W - 1 && room[h][w + 1] != c)
		return 0;
	return 1;
}

void print_room() {
	int i, j;
	for (i = 0; i < H; i++) {
		for (j = 0; j < W; j++) {
			printf("%c", room[i][j]);
		}
		printf("\n");
	}
	printf("\n");
}

void print_visited() {
	int i, j;
	for (i = 0; i < H; i++) {
		for (j = 0; j < W; j++) {
			printf("%d", visited[i][j]);
		}
		printf("\n");
	}
	printf("\n");
}

int main(void) {
	int i, j, ans = 0;
	scanf("%d%d%d%*c", &H, &W, &K);
	for (i = 0; i < H; i++) {
		for (j = 0; j < W; j++) {
			scanf("%c", &room[i][j]);
		}
		scanf("%*c");
	} 

	while (!check_finish()) {
		ans++;
		for (i = 0; i < H; i++) {
			for (j = 0; j < W; j++) {				
				if (room[i][j] == 'S' && !visited[i][j]) {
					room[i][j] = '.';
					replace(i, j, '.', 'S', K);
				}
			}
		}
		// print_room();
		// print_visited();
		clear_visited();

		for (i = 0; i < H; i++) {
			for (j = 0; j < W; j++) {
				if (room[i][j] == 'S')
					visit(i, j);
			}
		}

		for (i = 0; i < H; i++) {
			for (j = 0; j < W; j++) {
				if (visited[i][j] == 2) {
					replace(i - 1, j, '#', '.', K);
					replace(i + 1, j, '#', '.', K);
					replace(i, j - 1, '#', '.', K);
					replace(i, j + 1, '#', '.', K);
				}
			}
		}

		// print_room();
		// print_visited();
		clear_visited();
	}

	printf("%d\n", ans);
	return 0;
}