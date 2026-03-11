#define _CRT_SECURE_NO_WARNINGS
//#define _USE_MATH_DEFINES
#include<stdio.h>
//#include<stdlib.h>
//#include<math.h>
#include<string.h>
//#include<time.h>
//#define pi acos(-1.0)
#define P(type,x) fprintf(stdout,"%"#type"\n",x)
#define FALSE 0
#define TRUE 1
int num[9];
int used[10];
int dfs(int id);
int check();
int main() {
	int i;
	for (i = 0; i < 9; i++) {
		fscanf(stdin, "%d", &num[i]);
		if (num[i] != -1) used[num[i]] = TRUE;
	}
	P(d, dfs(0));
	return 0;
}

int dfs(int id) {
	if (id == 9) return check();
	if (num[id] != -1)return dfs(id + 1);
	int i, ans = 0;
	for (i = 1; i <= 9; i++) {
		if (used[i]) continue;
		num[id] = i, used[i] = TRUE;
		ans += dfs(id + 1);
		num[id] = -1, used[i] = FALSE;
	}
	return ans;
}

int check() {
	int sum = 0;
	sum += num[0] + num[2] + num[5] - num[8];
	sum += (num[1] + num[4] - num[7]) * 10;
	sum += (num[3] - num[6]) * 100;
	if (sum == 0) return 1;
	else return 0;
}