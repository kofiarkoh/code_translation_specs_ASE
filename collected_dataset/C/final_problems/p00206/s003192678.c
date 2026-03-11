#include <stdio.h>

int main(void)
{
	int l, m, n, c = 1;
	int trip = 0;
	int i;
	
	while (scanf("%d", &l), l != 0){
		for (i = 0; i < 12; i++){
			scanf("%d %d", &m, &n);
			trip += m - n;
			if (trip < l){
				c++;
			}
		}
		if (c < 12){
			printf("%d\n", c);
		}
		else {
			printf("NA\n");
		}
	}
	return (0);
}