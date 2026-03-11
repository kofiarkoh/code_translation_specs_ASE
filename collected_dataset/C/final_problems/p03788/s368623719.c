#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef char bool;
#define true 1
#define false 0

bool* boolFromChar( char* s, int size )
{
    bool* b = (bool *) malloc( size * sizeof( bool ) );
    for( int i = 0; i < size; i++ )
    {
        b[i] = s[i] == 'A';
    }
    return b;
}
bool* copyBool( bool*b, int size)
{
    bool* copyB = (bool *) malloc( size * sizeof( bool ) );
    memcpy( copyB, b, size );
    return copyB;
}
void printBool( bool* b, int size)
{
    for( int i = 0; i < size; i++ )
    {
        if( b[i] ) printf("A");
        else printf("B");
    }
}
int findHistorical( bool** hB, int hSize, bool* b, int size)
{
    for( int i = 0; i < hSize; i ++ )
    {
        if( memcmp( hB[i], b, size) == 0 )
        {
            return i;
        }
    }
    return -1;
}
int main(int argc, char **argv)
{
    int n, k, i, c, f, t, h;
    char *s;
	bool *b;
    bool *prevB;
    
    scanf("%d %d", &n, &k);

    s =  (char *) malloc( n * sizeof( char ) );
    bool **hB = (bool**) malloc( k * sizeof( bool*) );

    scanf("%s", s);
    
    b = boolFromChar(s, n);
    prevB = copyBool(b, n);
    for( i = 0; i < k; i++)
    {
        hB[i] = prevB;
        c = 0;
        f = -1;
        while( c >= 0 
               && c < n )
        {
            t = c;
            b[c] = !b[c];
            if( b[c] )
            {
                c += c-f;
            }
            else
            {
                c = f;
            }
            f = t;
        }
        //printBool( b, n);
        //printf("\n");
        h = findHistorical( hB, i, b, n);
        if( h >= 0 )
        {
            int loopSize = i-h;
            int remainingIteration = k-i;
            int index = remainingIteration%loopSize;
            printBool( hB[h+index], n);
            return 0;
        }
        prevB = copyBool(b, n);
    }
    printBool( b, n);
    free(s);
    free(b);
    free(prevB);
	return 0;
}
