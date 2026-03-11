#define _CRT_SECURE_NO_WARNINGS
//#define _USE_MATH_DEFINES
#include<stdio.h>
//#include<stdlib.h>
//#include<math.h>
//#include<string.h>
//#include<time.h>
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
char map[12][13];
int d[4] = { 0,-1,0,1 };
int ans;
void f(int, int);
int main() {
	int i, j;
	for (; ~fscanf(stdin, "%s", map[0]) != NULL; P(d, ans)) {
		getchar();
		ans = 0;
		for (i = 1; i < 12; i++) fscanf(stdin, "%s", map[i]);
		for (i = 0; i < 12; i++)
			for (j = 0; j < 12; j++)
				if (map[i][j] == '1') {
					ans++;
					f(i, j);
				}
	}
	return 0;
}

void f(int y, int x) {
	int i;
	map[y][x] = '0';
	for (i = 0; i < 4; i++) {
		if (0 <= x + d[i] && x + d[i] < 12 && map[y][x + d[i]] == '1') f(y, x + d[i]);
		if (0 <= y + d[i] && y + d[i] < 12 && map[y+d[i]][x] == '1') f(y+d[i], x);
	}
	return;
}