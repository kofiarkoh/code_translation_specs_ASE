#include<stdio.h>

int main()
{
	int a, b;
	int c[50000][5] = { 0 };
	scanf("%d%d", &a, &b);
	int l;
	for (int i = 1; i <= a; i++) {
		for (int j = 1; j <= 4; j++)
			scanf("%d", &c[i][j]);
		l = i ;
	}
	for (int i = 1; i <= l; i++) {
		if (c[i][3] != 0) {
			for (int j = 1; j <= l; j++) {
				if (c[j][2] == c[i][3]&&c[j][3]==0) {
					c[j][2] = 0;
					c[i][3] = 0;
				}
			}
		}
		if (c[i][4] != 0) {
			for (int j = 1; j <= l; j++) {
				if (c[j][1] == c[i][4]&&c[j][4]==0) {
					c[j][1] = 0;
					c[i][4] = 0;
				}
			}
		}
	}
	int n = 0;
	for (int i = 1; i <= l; i++) {
		for (int j = 3; j <= 4; j++)
			if (c[i][j] != 0)
				n++;
	}
	if (n == 0)
		printf("YES");
	else
		printf("NO");
	return 0;
}