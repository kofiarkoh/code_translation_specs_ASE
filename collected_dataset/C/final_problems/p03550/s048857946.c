#include<stdio.h>
int main(void){
    int n, i, j;
    long z, w, key, key2, key3, a[2002];
    scanf("%d %ld %ld", &n, &a[0], &a[1]);
    for ( i = 2; i < n + 2; i++ ) {
        scanf("%ld", &a[i]);
    }
    z = a[0];
    w = a[1];
    key = z - w;
    if ( key < 0 ) key *= -1;
        for ( i = 0; i < n + 2; i++ ) {
            if ( i == 1 ) i++;
            z = a[i];
            w = a[i + 1];
            if ( i == n + 1 ) w = a[1];
            key2 = z - w;
            if ( key2 < 0 ) key2 *= -1;
            for ( j = i + 1; j < n + 2; j++ ) {
                w = a[j];
                key3 = z - w;
                if ( key3 < 0 ) key3 *= -1; 
                if ( key3 < key2 ) key2 = key3;
            }
            if ( key2 > key ) key = key2;
        }
    printf("%ld\n", key);
    return 0;
}