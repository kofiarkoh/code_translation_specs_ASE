#include<stdio.h>
#define scanf_s scanf
int main(void) {
		int a, x = 0, t = 0;

		scanf_s("%d%d", &x, &t);

		a = x - t;
		if (a < 0)a = 0;

		printf("%d", a);
	return 0;
}