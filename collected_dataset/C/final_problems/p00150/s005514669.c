#include <stdio.h>
#include <math.h>

int prime[16384]; //10000ð´¦éÅ­Ì2Ì×«æ

int sosu ( int n ) {
	int i , lim = (int)(sqrt(n));
	if ( n < 2 ) return 0;
	for ( i = 2; i <= lim; i++ ) {
		if ( n % i == 0 ) return 0;
	}
	return n;
}

void tmp(void) {
	int i , j = 0;
	for ( i = 0; i <= 10000; i++ ) {
		if ( sosu(i) ) {
			prime[j] = sosu(i);
			j++;
		}
	}
	return;
}

int main (void) {
	int n , i , j;
	tmp();
	
	while (1) {
		int twin = 0;
		scanf("%d",&n);
		if ( n == 0 ) break;
		
		for ( i = 0; prime[i+1] < n; i++ ) {
			if ( prime[i+1] - prime[i] == 2 ) {
				twin = prime[i];
			}
		}
		printf("%d %d\n",twin,twin+2);
	}
	return 0;
}