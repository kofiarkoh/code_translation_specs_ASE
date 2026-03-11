#include <stdio.h>
#include <math.h>
 
#define MAX 1000000
 
char Prime[MAX];
 
void PrimeSearch_Eratos( void ) //エラトステネスの篩
{
    int i = 0, j = 0;
 
    for( i = 0 ; i < MAX ; i ++ ) Prime[i] = 1;
 
    Prime[0] = 0;
    Prime[1] = 0;
 
    for( i = 2 ; i <= sqrt( ( double )MAX ) + 1 ; i ++ ) //doubleにキャストしないと
    {                                                   //オーバーロードがなんたら
        if( !Prime[i] ) continue;                       //（　゜д゜）ポカン
 
        for( j = i * 2 ; j <= MAX ; j += i )
        {
            Prime[j] = 0;
        }
    }
 
    return ;
}
 
int main( void )
{
    int n = 0;
 
    PrimeSearch_Eratos();
 
    while( scanf( "%d", &n ) != EOF )
    {
        int i = 0;
        int Count = 0;
 
        for( i = 0 ; i <= n ; i ++ )
        {
            if( Prime[i] ) Count ++;
        }
 
        printf( "%d\n", Count );
    }
 
    return 0;
}