#include <stdio.h>

int main(int argc, char *argv[])
{
    int list[50][26], i, j, n, min;
    char buf[64];
    scanf("%d\n", &n);
    for ( i = 0; i < n; i++ ){
        fgets(buf, sizeof(buf), stdin);
        for ( j = 0; buf[j] != '\n'; j++ ){
            list[i][buf[j]-'a']++;
        }
    }
    for ( i = 0; i < 26; i++ ){
        for ( j = 0, min = 100; j < n; j++ ){
            if ( min > list[j][i] ){
                min = list[j][i];
            }
        }
        for ( j = 0; j < min; j++ ){
            printf("%c", i+'a');
        }
    }
    printf("\n");
    return 0;
}