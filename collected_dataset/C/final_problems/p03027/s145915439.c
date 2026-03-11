#include <stdio.h>
#include <string.h>
#include <limits.h>
#include <stdlib.h>
#include <math.h>

int main(void)
{
	long int q;
	long int x[100000], d[100000], n[100000];
	long int i, j;
	long int sum;
	long int work;
	
	scanf("%ld", &q);
	
	for(i = 0; i < q; i++) {
		scanf("%ld%ld%ld", &x[i], &d[i], &n[i]);
	}
	
	for(i = 0; i < q; i++) {
		sum = work = x[i];
		if(n[i] > 1000003) {
			for(j = 0; j < 1000003; j++) {
				if(sum > 1000003) {
					sum %= 1000003;
				}
				work += d[i];
				sum *= work;
			}
			for(j = 0; j < n[i] / 1000003; j++) {
				sum *= sum;
				if(sum > 1000003) {
					sum %= 1000003;
				}
			}
			work = x[i];
			for(j = 0; j < n[i] % 1000003; j++) {
				if(sum > 1000003) {
					sum %= 1000003;
				}
				work += d[i];
				sum *= work;
			}
		} else {
			for(j = 0; j < n[i] - 1; j++) {
				if(sum > 1000003) {
					sum %= 1000003;
				}
				work += d[i];
				sum *= work;
			}
		}
		printf("%ld\n", sum % 1000003);
	}
	
	return 0;
}