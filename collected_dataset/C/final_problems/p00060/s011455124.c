#include <stdio.h>

int main()
{
        int c1, c2, c3, i, cnt = 0;
        int card[11];

        for ( int i = 1 ; i < 11 ; i++ ) {
                card[i] = 1;
        }

        while ( scanf( "%d %d %d", &c1, &c2, &c3 ) == 3 ) {
                cnt = 0;
                for ( i = 1 ; i < 11 ; i++ ) {
                        if ( c1 != i && c2 != i && c3 != i ) {
                                if ( c1 + c2 + i <= 20 ) {
                                        cnt++;
                                }
                        }
                }
                if ( cnt >= 4 ) {
                        printf( "YES\n" );
                } else {
                        printf( "NO\n" );
                }
        }
        return 0;
}

