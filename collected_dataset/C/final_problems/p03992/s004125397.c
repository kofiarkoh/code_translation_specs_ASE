
#include <stdlib.h>
#include <stdio.h>
 
int main ( void ) {
    int i = 0;
 
    for (i = 0; i < 4; i++) {
         putchar(getchar());
    }
 
    putchar(' ');
 
    int c = getchar();
    while (c != '\n' && c != EOF) {
         putchar(c);
         c = getchar();
    } 
    putchar('\n');
 
    return EXIT_SUCCESS;
}
