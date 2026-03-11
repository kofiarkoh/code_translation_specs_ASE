#include <math.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
	double x, y, z, r;
} Coord;

typedef struct {
	int from, to;
	double weight;
} Edge;

Coord coord[ 100 ];
int node[ 100 ];
Edge edge[ 4950 ];

int compare( const void *a, const void *b ) {
	double at = (*(Edge *)a).weight, bt = (*(Edge *)b).weight;

	if ( at < bt )
		return -1;
	if ( at > bt )
		return 1;
	return 0;
}

int find( int x ) {
	if ( node[ x ] == x )
		return x;
	else
		return node[ x ] = find( node[ x ] );
}

int main( void ) {
	int i, j;
	int n;

	while ( scanf( "%d", &n ), n ) {
		int cnt;
		double sum;

		for ( i = 0; i < n; i++ )
			scanf( "%lf %lf %lf %lf", &coord[ i ].x, &coord[ i ].y, &coord[ i ].z, &coord[ i ].r );

		cnt = 0;
		for ( i = 0; i < n; i++ )
			for ( j = i + 1; j < n; j++ ) {
				edge[ cnt ].from = i;
				edge[ cnt ].to = j;
				edge[ cnt ].weight = fmax( sqrt( ( coord[ i ].x - coord[ j ].x ) * ( coord[ i ].x - coord[ j ].x ) + ( coord[ i ].y - coord[ j ].y ) * ( coord[ i ].y - coord[ j ].y ) + ( coord[ i ].z - coord[ j ].z ) * ( coord[ i ].z - coord[ j ].z ) ) - coord[ i ].r - coord[ j ].r, 0.0 );
				cnt++;
			}
		qsort( edge, cnt, sizeof(Edge), &compare );

		for ( i = 0; i < n; i++ )
			node[ i ] = i;

		sum = 0.0;
		for ( i = 0; i < cnt; i++ ) {
			int set1 = find( edge[ i ].from ), set2 = find( edge[ i ].to );
			if ( set1 != set2 ) {
				node[ set2 ] = set1;
				sum += edge[ i ].weight;
			}
		}

		printf( "%.3f\n", sum );
	}

	return 0;
}