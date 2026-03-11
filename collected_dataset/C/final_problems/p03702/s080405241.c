#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[])
{
    char buf[512];
    int n, i, list[100000] = {0};
    int max = 0, max_i, a, b, attack = 0;

    fgets(buf, sizeof(buf), stdin);
    n = atoi( strtok(buf, " \n") );
    a = atoi( strtok(NULL, " \n") );
    b = atoi( strtok(NULL, " \n") );

    for ( i = 0; i < n; i++ ){
        fgets(buf, sizeof(buf), stdin);
        list[i] = atoi(buf);
        if ( list[i] > max ){
            max = list[i];
            max_i = i;
        }
    }


    while( max != 0 ){
        int j;
        max = 0;
        for ( i = 0; i < n; i++ ){
            if ( i == max_i ){
                list[i] -= a;
            } else {
                list[i] -= b;
            }
            if ( list[i] > max ){
                max = list[i];
                j = i;
            }
        }
        max_i = j;
        attack++;
    }

    printf("%d\n", attack);

    return 0;
}
