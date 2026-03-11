#include <stdio.h>

int c10, c50, c100, c500;
int a10, a50, a100, a500;
int b_mon;
int a_mon;
int price;

int main(void) 
{
	scanf("%d",&price);

	while (price) {
		scanf("%d %d %d %d", &c10, &c50, &c100, &c500);
		b_mon = c10 * 10 + c50 * 50 + c100 * 100 + c500 * 500;
		a_mon = b_mon - price;

		a500 = a_mon / 500;
		a_mon %= 500;
		a100 = a_mon / 100;
		a_mon %= 100;
		a50 = a_mon / 50;
		a_mon %= 50;
		a10 = a_mon / 10;
		a_mon %= 10;

		if (a10 < c10) {
			printf("10 %d\n",c10 - a10);
		}
		if (a50 < c50) {
			printf("50 %d\n",c50 - a50);
		}
		if (a100 < c100) {
			printf("100 %d\n",c100 - a100);
		}
		if (a500 < c500) {
			printf("500 %d\n",c500 - a500);
		}
                puts("");
		scanf("%d",&price);
	}
	return 0;
}