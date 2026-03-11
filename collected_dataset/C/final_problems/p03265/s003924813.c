#include<stdio.h>
#include<stdlib.h>
int main() {
	int x[5];
	int y[5];
	int a, b;
	scanf("%d %d %d %d", &x[1], &y[1], &x[2], &y[2]);
//	if (y[1] < y[2]) {
		a = x[2] - x[1];
		b = y[2] - y[1];
		x[4] = x[1] - b;
		y[4] = y[1] + a;
		y[3] = y[2] + a;
		x[3] = x[2] - b;
	//}
	//else {

	//}

	printf("%d %d %d %d\n", x[3], y[3], x[4], y[4]);
}