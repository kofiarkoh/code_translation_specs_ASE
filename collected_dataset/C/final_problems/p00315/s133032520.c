#include<stdio.h>


#define N_MAX 10000


int SameOrDiff_H( char* ,int* ) ;
int SameOrDiff_V( char(*)[ N_MAX ] ,int* ,int* ) ;

char pixel[ N_MAX ][ N_MAX ] ;

int main( void )
{
	int C ,N ;
	scanf( "%d %d" ,&C ,&N ) ;

	int cnt ;

	int i ,j ;
	for( i = cnt = 0 ; i < N ; ++i )
	{
		scanf( "%s" ,pixel[ i ] ) ;

		if( cnt == 0 && SameOrDiff_H( pixel[ i ] ,&N ) == 1 )
		{
			++cnt ;
		}
	}

	for( i = 0 ; cnt == 0 && i < N ; ++i )
	{
		if( SameOrDiff_V( pixel ,&i ,&N ) == 1 )
		{
			++cnt ;
		}
	}

	int n ;
	int Y[ 100 ] ,X[ 100 ] ;
	int y ,x ;
	for( i = 1 ; i < C ; ++i )
	{
		int same = cnt ;
		int Yp = 0 ;
		int Xp = 0 ;
		int sameY[ N_MAX + 1 ] = { 0 } ;
		int sameX[ N_MAX + 1 ] = { 0 } ;

		scanf( "%d" ,&n ) ;
		for( j = 0 ; j < n ; ++j )
		{
			scanf( "%d %d" ,&y ,&x ) ;
			/*if( sameY[ y ] == 0 )
			{
				Y[ Yp++ ] = y - 1 ;

				sameY[ y ] = 1 ;
			}
			if( sameX[ x ] == 0 )
			{
				X[ Xp++ ] = x - 1 ;

				sameX[ x ] = 1 ;
			}*/

			pixel[ y - 1 ][ x - 1 ] = pixel[ y - 1 ][ x - 1 ] == '1' ? '0' : '1' ;
		}

		for( j = 0 ; cnt == same && j < N ; ++j )
		{
			if( (SameOrDiff_H( pixel[ j ] ,&N ) | SameOrDiff_V( pixel ,&j ,&N )) == 1 )
			{
				++cnt ;
			}
		}

		/*for( j = 0 ; j < Yp && cnt == same ; ++j )
		{
			if( SameOrDiff_H( pixel[ Y[ j ] ] ,&N ) == 1 )
			{
				++cnt ;
			}
		}

		for( j = 0 ; j < Xp && cnt == same ; ++j )
		{
			if( SameOrDiff_V( pixel ,&X[ j ] ,&N ) == 1 )
			{
				++cnt ;
			}
		}*/
	}

	printf( "%d\n" ,C - cnt ) ;

	return 0 ;
}

int SameOrDiff_H( char *Pixel ,int *Length )	// horizontal
{
	int Left = 0 ;
	int Right = *Length - 1 ;

	while( Left < Right )
	{
		if( Pixel[ Left++ ] != Pixel[ Right-- ] )
		{
			return 1 ;
		}
	}

	return 0 ;
}

int SameOrDiff_V( char ( *Pixel )[ N_MAX ] ,int *X ,int *Length )	// vertical
{
	int Left = 0 ;
	int Right = *Length - 1 ;

	while( Left < Right )
	{
		if( Pixel[ Left++ ][ *X ] != Pixel[ Right-- ][ *X ] )
		{
			return 1 ;
		}
	}

	return 0 ;
}
