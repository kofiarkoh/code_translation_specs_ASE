#include <stdio.h>


int main(void){
	int A, B, C, X ;
	fscanf(stdin, "%d\n%d\n%d\n%d", &A, &B, &C, &X ) ;

	int ways = 0 ;
	int a, b, c ;
	int max_a, max_b ;

	max_a = X / 500 ;
	if( max_a > A) {
		max_a = A ;
	}
	for( a = max_a ; a >= 0 ; a-- ) {
		max_b = ( X - 500 * a ) / 100 ;
		if( max_b > B ) {
			max_b = B ;
		}
		for( b = max_b ; b >= 0 ; b-- ) {
			c = ( X - 500 * a - 100 * b ) / 50 ;
			if( c > C ) {
				break ;
			}
			ways++ ;
		}
	}
	printf("%d", ways ) ;
	return 0 ;
}
