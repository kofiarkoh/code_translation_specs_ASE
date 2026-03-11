#include <stdio.h>
#include <stdlib.h>

#define MAX_N 100291

int isntPrime[ MAX_N + 1 ] = { 1, 1, 0 };

void quickSort( int *, int, int );
int median( int, int, int );
void insertionSort( int *, int );

int main( void ) {
	int i, j, N, P, cnt, p[ 22 ], sum[ 253 ];

	for ( i = 4; i <= MAX_N; i += 2 )
		isntPrime[ i ] = 1;

	for ( i = 3; i * i <= MAX_N; i += 2 )
		if ( !isntPrime[ i ] )
			for ( j = i * 2; j <= MAX_N; j += i )
				isntPrime[ j ] = 1;

	for ( ; scanf( "%d %d", &N, &P ), N != -1; printf( "%d\n", sum[ P - 1 ] ) ) {
		cnt = 0;
		for ( i = N + 1; cnt < 22; i++ )
			if ( !isntPrime[ i ] )
				p[ cnt++ ] = i;

		cnt = 0;
		for ( i = 0; i < 22; i++ )
			for ( j = i; j < 22; j++ )
				sum[ cnt++ ] = p[ i ] + p[ j ];

		quickSort( sum, 0, 176 );
	}

	return 0;
}

void quickSort( int *data, int l, int r ) {
	if ( l < r ) {
		int i = l, j = r, pivot = median( data[ i ], data[ ( i + j ) / 2 ], data[ j ] ), tmp;

		while ( 1 ) {
			for ( ; data[ i ] < pivot; i++ ) ;
			for ( ; pivot < data [ j ]; j-- ) ;
			if ( i >= j )
				break;

			tmp = data[ i ];
			data[ i ] = data[ j ];
			data[ j ] = tmp;

			i++;
			j--;
		}

		if ( i - l < 20 )
			insertionSort( data + l, i - l );
		else
			quickSort( data, l, i - 1 );

		if ( r - j < 20 )
			insertionSort( data + j + 1, r - j );
		else
			quickSort( data, j + 1, r );
	}
}

int median( int x, int y, int z ) {
	if ( x < y )
		if ( y < z )
			return y;
		else if ( z < x )
			return x;
		else
			return z;
	else if ( z < y )
		return y;
	else if ( x < z )
		return x;
	else
		return z;
}

void insertionSort( int *data, int n ) {
	int i, j, tmp;

	for ( i = 1; i < n; i++ ) {
		tmp = data[ i ];

		for ( j = i; j && data[ j - 1 ] > tmp; j-- )
			data[ j ] = data[ j - 1 ];

		data[ j ] = tmp;
	}
}