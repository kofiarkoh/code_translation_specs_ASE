#include <stdio.h>
#include <string.h>
#include <limits.h>
#include <stdlib.h>
#include <math.h>


int main(void)
{
	long int i, j, k;
	long int n;
	long int sum;
	long int max;
	long int count;
	long int index;
	long int a[26][5001] = {{0}};
	char str[5001];
	
	scanf("%ld%s", &n, str);
	
	for(i = 0; i < n; i++) {
		a[str[i] - 'a'][0]++;
		a[str[i] - 'a'][a[str[i] - 'a'][0]] = i;
	}
	
	max = sum = 0;
	
	for(i = 0; i < n; i++) {
		index = 1;
		for(j = 2; j <= a[str[i] - 'a'][0]; j++) {
			if( a[str[i] - 'a'][ j ] > i ) {
				sum = 1;
				for(k = 1; k + a[ str[i] - 'a' ][ j ] < n; k++) {
					if( (str[i + k] != str[ a[str[i] - 'a'][ j ] + k ]) || ( i + k >= a[str[i] - 'a'][j] ) ) {
						break;
					}
					sum++;
				}
				if(index < k) {
					index = k;
				}
				if(max < sum) {
					max = sum;
				}
			}
		}
		//i += (index - 1);
	}
	
	printf("%ld\n", max);
	
	return 0;
}
