#include <stdio.h>
#include <string.h>
#include <limits.h>
#include <stdlib.h>
#include <math.h>


int main(void)
{
	long int n;
	long int i, j;
	long int sum_x = 0;
	long int sum_y = 0;
	long int x[100000];
	long int y[100000];
	long int flg_x[100000] = {0};
	long int flg_y[100000] = {0};
	
	scanf("%ld", &n);
	
	for(i = 0; i < n; i++) {
		scanf("%ld%ld", &x[i], &y[i]);
		flg_x[x[i] - 1]++;
		flg_y[y[i] - 1]++;
	}
	
	if(n < 3) {
		printf("0\n");
		return 0;
	}
	
	for(i = 0; i < 100000; i++) {
		if(flg_x[i] != 0) {
			sum_x++;
		}
		if(flg_y[i] != 0) {
			sum_y++;
		}
	}
	
	if( (sum_x == 0) || (sum_y == 0) ) {
		printf("0\n");
		return 0;
	}
	
	printf("%ld\n", sum_x * sum_y - n);
	
	return 0;
}