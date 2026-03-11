#include <stdio.h>
#include <stdlib.h>

int main(void)
{
        char x[20], y[20];
        unsigned long long int xi, yi;
        int count = 0;

        scanf("%s %s", x, y);

        xi = (unsigned long long int)atol(x);
        yi = (unsigned long long int)atol(y);

        do{
                ++count;
                xi <<= 1;
        }while(xi <= yi);

        printf("%d\n", count);

        return 0;
}
