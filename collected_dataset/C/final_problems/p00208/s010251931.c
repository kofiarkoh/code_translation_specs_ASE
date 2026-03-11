#include <stdio.h>

const int conv[8] = {0, 1, 2, 3, 5, 7, 8, 9};

int main(void)
{
	int n;
	int f;
	int x;
	int i;
	
	while (scanf("%d", &n), n != 0){
		f = 0;
		for (i = 27; i >= 0; i -= 3){
			x = n >> i & 7;
			if (x != 0){
				f = 1;
			}
			if (f){
				printf("%d", conv[x]);
			}
		}
		printf("\n");
	}
}