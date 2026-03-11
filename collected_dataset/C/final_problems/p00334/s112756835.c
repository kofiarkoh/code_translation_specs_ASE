#include<stdio.h>


int check[ '1' * 10  +  '1' * 6  +  '1' * 3  +  1 ] ;


int ans ;



int min( int a ,int b )
{
	return a > b ? b : a ;
}


int max( int a ,int b )
{
	return a > b ? a : b ;
}



int key( int a ,int b ,int c )
{
	char str[ 16 ] ;

	sprintf( str ,"%4d%4d%4d" ,a ,b ,c ) ;


	a = 0 ;


	int i ;
	for( i = 12 ; i > 0 ; --i )
	{
		if( str[ i ] == '\0' )
		{
			a += ( '0' * i ) ;
		}

		else
		{
			a += ( str[ i ] * i ) ;
		}
	}


	return a ;
}

int main()
{
	int n ;
	scanf( "%d" ,&n ) ;


	int a ,b ,c ;

	int i ;
	for( i = 0 ; i < n ; ++i )
	{
		scanf( "%d %d %d" ,&a ,&b ,&c ) ;


		int aa = min(  a ,min( b ,c )  ) ;

		int bb = min(  max( a ,b ) ,min(  max( b ,c ) ,max( a ,c )  )  ) ;

		int cc = max(  a ,max( b ,c )  ) ;


		ans += check[ key( aa ,bb ,cc ) ] ;

		
		check[ key( aa ,bb ,cc ) ] = 1 ;
	}


	printf( "%d\n" ,ans ) ;


	return 0 ;
}