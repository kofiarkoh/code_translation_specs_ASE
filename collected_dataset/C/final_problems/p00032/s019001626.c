#include <stdio.h>

int main(void)
{
    int a, b, c;
    int square, diamond;

    square = diamond = 0;
    while (scanf("%d,%d,%d", &a, &b, &c) != EOF){
        if (a * a + b * b == c * c){
            square++;
        }
        else if (a == b){
            diamond++;
        }
    }

    printf("%d\n", square);
    printf("%d\n", diamond);

    return (0);
}