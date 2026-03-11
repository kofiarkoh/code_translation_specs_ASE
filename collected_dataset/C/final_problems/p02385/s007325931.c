#define _CRT_SECURE_NO_WARNINGS
#define _USE_MATH_DEFINES
#include<stdio.h>
void Dice(char p, int []);
int Count(int[]);

int main() {
	int d[6], d2[6], i, flg = 0;
	char str[2][4] = { "No","Yes" };
	for (i = 0; i < 6; i++)  scanf("%d", &d[i]);
	for (i = 0; i < 6; i++)  scanf("%d", &d2[i]);
	if (Count(d) >= 10) { printf("Yes\n"); return 0; }
	if (d[0] == d2[0]);
	else if (d[5] == d2[0]) Dice('N', d), Dice('N', d);
	else if (d[1] == d2[0]) Dice('N', d);
	else if (d[4] == d2[0]) Dice('S', d);
	else if (d[2] == d2[0]) Dice('W', d);
	else if (d[3] == d2[0]) Dice('E', d);
	if (d[1] == d2[1]);
	else if (d[4] == d2[1]) Dice('L', d), Dice('L', d);
	else if (d[2] == d2[1]) Dice('R', d);
	else if (d[3] == d2[1]) Dice('L', d);
	for (i = 3; i < 6; i++) {
		if (d[i] != d2[i]) break;
		if(i==5) flg = 1;
	}
    printf("%s\n", str[flg]);
	return 0;
}

void Dice(char p, int d[]) {
	int tmp;
	if (p == 'S') { tmp = d[0]; d[0] = d[4]; d[4] = d[5]; d[5] = d[1]; d[1] = tmp; }
	if (p == 'W') { tmp = d[0]; d[0] = d[2]; d[2] = d[5]; d[5] = d[3]; d[3] = tmp; }
	if (p == 'E') { tmp = d[0]; d[0] = d[3]; d[3] = d[5]; d[5] = d[2]; d[2] = tmp; }
	if (p == 'N') { tmp = d[0]; d[0] = d[1]; d[1] = d[5]; d[5] = d[4]; d[4] = tmp; }
	if (p == 'L') { tmp = d[1]; d[1] = d[3]; d[3] = d[4]; d[4] = d[2]; d[2] = tmp; }
	if (p == 'R') { tmp = d[1]; d[1] = d[2]; d[2] = d[4]; d[4] = d[3]; d[3] = tmp; }
}

int Count(int d[]){
	int cnt = 0, i, j;
	for (i = 0; i < 5; i++) {
		for (j = i + 1; j < 6; j++)
			if (d[i] == d[j])++cnt;
	}
	return cnt;
}