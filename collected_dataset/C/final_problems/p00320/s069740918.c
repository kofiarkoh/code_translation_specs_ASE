#include<stdio.h>

int main()
{
	int a ,b ;
	scanf( "%d %d" ,&a ,&b ) ;

	int handan = a ;
	int seki = a * b ;
	int same = 1 ;
	int different = 0 ;

	int i ;
	for( i = 1 ; i < 6 ; i++ )
	{
		scanf( "%d %d" ,&a ,&b ) ;

		if( seki == a * b )
		{
			if( handan == a || handan == b )
			{
				same++ ;
			}
			else
			{
				different++ ;
			}
		}
		else
		{
			different++ ;
		}
	}


	if( ( same == 4 && different == 2 ) || ( same == 2 && different == 4 ) || same == 6 )
	{
		puts( "yes" ) ;
	}
	else
	{
		puts( "no" ) ;
	}


	return 0 ;
}