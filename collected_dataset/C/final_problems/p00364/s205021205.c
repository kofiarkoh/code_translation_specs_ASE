#include<stdio.h>


int main( void )
{
	int N ,t ;
	scanf( "%d %d" ,&N ,&t ) ;

	int x ,y ;
	double maxH = 0 ;
	
	int i ;
	for( i = 0 ; i < N ; ++i )
	{
		scanf( "%d %d" ,&x ,&y ) ;

		double high = (1.0 / (double)x) * (double)y ;
		if( high > maxH )
		{
			maxH = high ;
		}
	}

	printf( "%lf\n" ,maxH * t ) ;

	return 0 ;
}
