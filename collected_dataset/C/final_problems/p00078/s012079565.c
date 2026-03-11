#include<stdio.h>
int main(void){
    int n, x, y, key, i, j;
    scanf("%d", &n);
    while ( n ) {
        int a[100][100] = { 0 };
        y = n / 2 + 2;
        x = n / 2 + 1;
        for ( i = 1; i <= n * n; i++ ) {
            key = 1;
            while ( key ){
                key = 0;
                if ( y > n ) { y = 1; key++;}
                else if ( y < 1 ) { y = n; key++;}
                if ( x > n ) { x = 1; key++;}
                else if ( x < 1 ) { x = n; key++;}
                if ( a[y][x] != 0 ) { y++; x--; key++;}
            }
            a[y][x] = i;
            /*printf("%d, x%d y%d\n", a[y][x], x, y);*/
            y++;
            x++;
        }
        for ( i = 1; i <= n; i++ ) {
            for ( j = 1; j <= n; j++ ) {
                printf("%4d", a[i][j]);
            }
            printf("\n");
        }
        scanf("%d", &n);
    }
    return 0;
}