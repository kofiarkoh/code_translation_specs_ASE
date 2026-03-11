#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	char h;
	char w;
	char direction;
} Point;

Point find(Point current, char direction, char** array, int H, int W) {
	Point target;
	target.h = -1;
	target.w = -1;
	target.direction = -1;
	if (direction == 'U') {
		for (int h = current.h - 1; h >= 0; h--) {
			if (array[h][current.w] == 'o') {
				target.h = h;
				target.w = current.w;
				target.direction = 'U';
				return target;
			}
		}
	}
	if (direction == 'D') {
		for (int h = current.h + 1; h < H; h++) {
			if (array[h][current.w] == 'o') {
				target.h = h;
				target.w = current.w;
				target.direction = 'D';
				return target;
			}
		}
	}
	if (direction == 'L') {
		for (int w = current.w - 1; w>=0; w--) {
			if (array[current.h][w] == 'o') {
				target.h = current.h;
				target.w = w;
				target.direction = 'L';
				return target;
			}
		}
	}
	if (direction == 'R') {
		for (int w = current.w + 1; w < W; w++) {
			if (array[current.h][w] == 'o') {
				target.h = current.h;
				target.w = w;
				target.direction = 'R';
				return target;
			}
		}
	}
	return target;
}

// N number of all o and frog
void dfs(Point current, char** array, int H, int W, int N, char * result) {
	if (N == 1) {
		result[0] = '\0';
		return;
	}
	Point target;
	//jump left
	if (current.direction != 'R') {
		target = find(current, 'L', array, H, W);
		// exist target
		if (target.h != -1) {
			array[current.h][current.w] = '.';
			dfs(target, array, H, W, N - 1, result);
			// put back the o
			array[current.h][current.w] = 'o';
			if (strlen(result) == N - 2) {
				char tmp[30];
				sprintf(&tmp, "%c%s", target.direction, result);
				strcpy(result, tmp);
				return;
			}
		}
	}
	//jump right
	if (current.direction != 'L') {
		target = find(current, 'R', array, H, W);
		// exist target
		if (target.h != -1) {
			array[current.h][current.w] = '.';
			dfs(target, array, H, W, N - 1, result);
			// put back the o
			array[current.h][current.w] = 'o';
			if (strlen(result) == N - 2) {
				char tmp[30];
				sprintf(&tmp, "%c%s", target.direction, result);
				strcpy(result, tmp);
				return;
			}
		}
	}
	//jump up
	if (current.direction != 'D') {
		target = find(current, 'U', array, H, W);
		// exist target
		if (target.h != -1) {
			array[current.h][current.w] = '.';
			dfs(target, array, H, W, N - 1, result);
			// put back the o
			array[current.h][current.w] = 'o';
			if (strlen(result) == N - 2) {
				char tmp[30];
				sprintf(&tmp, "%c%s", target.direction, result);
				strcpy(result, tmp);
				return;
			}
		}
	}
	//jump down
	if (current.direction != 'U') {
		target = find(current, 'D', array, H, W);
		// exist target
		if (target.h != -1) {
			array[current.h][current.w] = '.';
			dfs(target, array, H, W, N - 1, result);
			// put back the o
			array[current.h][current.w] = 'o';
			if (strlen(result) == N - 2) {
				char tmp[30];
				sprintf(&tmp, "%c%s", target.direction, result);
				strcpy(result, tmp);
				return;
			}
		}
	}
}

// avoid returning a pointer to local data
int main() {
	int H, W;
	int N = 0;
	char ** array;
	char direction;
	Point current;
	scanf("%d %d\n", &H, &W);
	array = (char **)malloc(H * sizeof(char *));
	for (int h = 0; h < H; h++) {
		array[h] = (char *) malloc(W * sizeof(char));
	}

	for (int h = 0; h < H; h++) {
		for (int w = 0; w < W; w++) {
			array[h][w] = getchar();
			if (array[h][w] != 'o' && array[h][w] != '.') {
				//frog
				current.h = h;
				current.w = w;
				current.direction = array[h][w];
				N++;
			}
			if (array[h][w] == 'o') {
				N++;
			}
		}
		getchar();
	}
	char result[30];
	dfs(current, array, H, W, N, result);
	printf("%s\n", result);
	for (int h = 0; h < H; h++) {
		free(array[h]);
	}
	free(array);
	getchar();
	return 0;
}