#include <stdio.h>

int main(void)
{
	int n;
	int i, j;
	int num[5];
	char a[8][5];
	char abacus[8][10] = {
		"*****     ",
		"     *****",
		"==========",
		" **** ****",
		"* **** ***",
		"** **** **",
		"*** **** *",
		"**** **** ",
	};
	
	while (scanf("%d", &n) != EOF){
		for (i = 0; i < 5; i++){
			num[4 - i] = n % 10;
			n /= 10;
		}
		for (i = 0; i < 8; i++){
			for (j = 0; j < 5; j++){
				a[i][j] = abacus[i][num[j]];
			}
		}
		for (i = 0; i < 8; i++){
			for (j = 0; j < 5; j++){
				printf("%c", a[i][j]);
			}
			printf("\n");
		}
		printf("\n");
	}
	
	return (0);
}