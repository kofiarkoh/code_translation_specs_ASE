#include<stdio.h>



char check[ 1001 ][ 1000 ][ 999 ] ;


int ans ;



int min( int a ,int b )
{
	return a > b ? b : a ;
}


int max( int a ,int b )
{
	return a > b ? a : b ;
}



int main()
{
	int n ;
	scanf( "%d" ,&n ) ;


	int a ,b ,c ;

	int aa ,bb ,cc ;

	int i ;
	for( i = 0 ; i < n ; ++i )
	{
		scanf( "%d %d %d" ,&a ,&b ,&c ) ;


		aa = min(  a ,min( b ,c )  ) ;

		bb = min(  max( a ,b ) ,min(  max( b ,c ) ,max( a ,c )  )  ) ;

		cc = max(  a ,max( b ,c )  ) ;



		if( check[ aa ][ bb ][ cc ] != '\0' )
		{
			ans += check[ aa ][ bb ][ cc ] ;
		}

		else
		{		
			++check[ aa ][ bb ][ cc ] ;
		}
	}


	printf( "%d\n" ,ans ) ;


	return 0 ;
}