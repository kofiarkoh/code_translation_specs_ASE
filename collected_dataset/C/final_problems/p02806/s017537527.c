#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define Nmax 50

int main(int argc, char const *argv[])
{
    /* code */
    int N = 0;
    scanf("%d\n", &N);

    int i = 0;
    char s[Nmax][100];
    int t[Nmax];
    for(i = 0; i < N; i ++)
    {
        scanf("%s %d\n", &s[i], &t[i]);
    }
    char X[100];
    scanf("%s", &X);
    
    /*
    printf("%d\n", N);
    for(i = 0; i < N; i ++)
    {
        printf("%s %d\n", s[i], t[i]);
    }
    printf("%s\n", X);
    */

    int T = 0;
    for(i = N - 1; i >= 0; i --)
    {
        if(strcmp(X, s[i]) != 0)
        {
            T += t[i];
        }
        else
        {
            break;
        }
        
    }
    printf("%d\n", T);
    return 0;
}