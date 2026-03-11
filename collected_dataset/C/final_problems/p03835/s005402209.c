#include <stdio.h>
#include <stdlib.h>

int main(void) {
    char str[16];
    int x, y, z, s, k, c=0;

    scanf("%d %d", &k, &s);
    if(k > s) k = s;

    for(x=0; x < k+1; x++) {
        for(y=0; y < k+1; y++) {
            for(z=0; z < k+1; z++) {
                if((x+y+z) == s) c++;
            }
        }
    }
    printf("%d", c);
}