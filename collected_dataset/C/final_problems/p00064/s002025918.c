#include <stdio.h>
#include <ctype.h>

int main()
{
        char str[81], tmp[6];
        int num = 0, i, sum = 0;

        while ( scanf( "%s", str ) == 1 ) {
                i = 0;
                while ( str[i] ) {
                        if ( isdigit( str[i] ) ) {
                                num = 0;
                                while ( isdigit( str[i] ) ) {
                                        num = 10 * num + ( str[i] - '0' );
                                        i++;
                                }
                                sum = sum + num;
                        } else {
                                i++;
                        }
                }
        }

        printf( "%d\n", sum );

        return 0;
}

