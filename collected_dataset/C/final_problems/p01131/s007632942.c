#include <stdio.h>
#include <string.h>

int main( void ) {
	int n, sum;
	char ch[ 9 ][ 6 ] = { ".,!? ", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz" }, key, tmp;

	scanf( "%d ", &n );
	while ( n-- ) {
		while ( ( key = getchar() ) != '\n' ) {
			if ( key == '0' )
				continue;

			for ( sum = 0; ( tmp = getchar() ) != '0'; sum++ ) ;

			putchar( ch[ key - '1' ][ sum % strlen( ch[ key - '1' ] ) ] );
		}

		putchar( '\n' );
	}

	return 0;
}