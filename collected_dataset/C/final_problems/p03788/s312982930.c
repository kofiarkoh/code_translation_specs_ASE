#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int changeState( int f, int c, char *s)
{
    if( *s == 'A')
    {
        *s = 'B';
        c = f;
    }
    else
    {
        *s = 'A';
        c += c-f;
    }
    return c;
}

int main(int argc, char **argv)
{
    int n, k, i, c, f, t;
    char *s;
	scanf("%d %d", &n, &k);
    s =  (char *) malloc( n * sizeof( char ) );
    scanf("%s", s);
    bool *b = (bool *) malloc( n* sizeof( bool ) );
    for( i =0; i < n; i ++)
    {
        b[i] = s[i] == 'A';
    }
    for( i = 0; i < k; i++)
    {
        c = 0;
        f = -1;
        while( c >= 0 
               && c < n )
        {
            t = c;
            if( b[c] )
            {
                b[c] = false;
                c = f;
            }
            else
            {
                b[c] = true;
                c += c-f;
            }
            f = t;
        }
    }
    for( i =0; i < n; i ++)
    {
        if( b[i] )
        {
            s[i] = 'A';
        }
        else
        {
            s[i] = 'B';
        }
    }
    printf("%s", s);
    free(s);
	return 0;
}
