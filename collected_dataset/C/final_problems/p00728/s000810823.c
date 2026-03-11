#include <stdio.h>
#define MX 1000
#define MN 0
#define EXN 2

int main(void){
    int i, n, s, sum, mn, mx;
    
    do
    {
        sum = 0;
        mn = MX;
        mx = MN;
        
        scanf( "%d", &n );
        if( n != 0 )
        {
            for( i = 0; i < n; i++ )
            {
                scanf( "%d", &s );
                sum += s;
                
                if( mx < s ) mx = s;
                if( mn > s ) mn = s;
            }
            
            if( mx == MN ) mx = 0;
            if( mn == MX ) mn = 0;
            
            sum = ( sum - ( mx + mn ) ) / ( n - EXN );
            printf( "%d\n", sum );
        }
    }while( n != 0 );
}
