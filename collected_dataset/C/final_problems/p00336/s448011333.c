#include<stdio.h>
#include<string.h>

#define MOD % 1000000007
#define LEN 1000 + 1


int main( void )
{
	char str[LEN] ,spell[LEN] ;
	scanf( "%s %s" ,str ,spell ) ;

	int st_len = strlen( str ) ;
	int sp_len = strlen( spell ) ;

	int ans[LEN] = { 1 } ;
	int i ,j ;
	for( i = 0 ; i < st_len ; ++i )
	{
		for( j = sp_len - 1 ; j >= 0 ; --j )
		{
			if( str[ i ] == spell[ j ] )
			{
				ans[ j + 1 ] = (ans[ j ] + ans[ j + 1 ]) MOD ;
			}
		}
	}

	printf( "%d\n" ,ans[ sp_len ] ) ;

	return 0 ;
}
