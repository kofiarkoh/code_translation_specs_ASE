#include<stdio.h>
#include<string.h>



long speller( char *fake ,char *now ,int end )
{
	int ret = 0 ;

	int fake_p = 0 ,now_p = 0 ;


	if( now[ now_p ] < end )
	{
		return 1 ;
	}


	while( fake[ fake_p ] != '\0' )
	{
		if( fake[ fake_p ] == now[ now_p ] )
		{
			ret += speller( fake + fake_p + 1 ,now + 1 ,end - 1 ) ;

			now_p ;


			if( now[ now_p ] == '\0' )
			{
				return ret ;
			}
		}


		++fake_p ;
	}


	return ret ;	
}




int main()
{
	char fake_spell[ 1000 + 1 ] ;
	scanf( "%s" ,fake_spell ) ;

	char spell[ 1000 + 1 ] ;
	scanf( "%s" ,spell ) ;


	int end = strlen( fake_spell ) - strlen( spell ) + 1 ;


	printf( "%ld\n" ,speller( fake_spell ,spell ,end ) ) ;



	return 0 ;
}