#include<stdio.h>
int main()
{
	int x, i, a;
	scanf("%d", &x);
	for (i = 1; i <= 100000; i++) {
		if (x >= 100 * i && x <= 105 * i) {
			printf("1\n");
			break;
		}
		else if (i * 100 >= x) {
			printf("0\n");
			break;
		}
	}
	return 0;
}