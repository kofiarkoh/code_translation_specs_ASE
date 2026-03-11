#include<stdio.h>

int main()

{
	int n;

	scanf("%d", &n);

	if (n < 3 * 5 * 7) {
		printf("0\n");

	}
	else if (n < 3 * 5 * 9) {
		printf("1\n");

	}
	else if (n < 3 * 5 * 11) {
		printf("2\n");
	}
	else if (n < 3 * 7 * 9) {

		printf("3\n");

	}
	else if (n < 3 * 5 * 13) {

		printf("4\n");

	}
	else {
		printf("5\n");

	}

	return 0;



}