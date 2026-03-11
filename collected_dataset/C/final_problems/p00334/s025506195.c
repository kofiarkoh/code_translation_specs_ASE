#include<stdio.h>


int check[ 111000 + 1 ] ;


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
	a *= 100 ;

	b *= 10 ;


	return a + b + c ;
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