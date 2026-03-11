#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct _data {
        int pre;
        int this;
        int trading_time;
} data_t;

int main()
{
        data_t d[1000];
        int i = 0, c, di;
        char * p, tmp1[5], tmp2[3], tmp[8];
        while ( i < 1000 ) {
                d[i].pre = 0;
                d[i].this = 0;
                d[i].trading_time = 0;
                i++;
        }

        while ( fgets( tmp, 8, stdin ) != NULL ) {
                if ( tmp[0] == '\n' ) { break; }

                i = 0;
                while ( tmp[i] != ',' ) {
                        tmp1[i] = tmp[i];
                        i++;
                }
                tmp1[i] = '\0';
                c = atoi( tmp1 );
                di = atoi( &tmp[i+1] );
                d[c].pre = di;
                d[c].trading_time++;
        }


        while ( scanf( "%d,%d", &c, &di ) == 2 ) {
                d[c].this = di;
                d[c].trading_time++;
        }

        for ( i = 0 ; i < 1000 ; i++ ) {
                if ( d[i].this > 0 && d[i].pre > 0 ) {
                        printf( "%d %d\n", i, d[i].trading_time );
                }
        }

        return 0;
}

