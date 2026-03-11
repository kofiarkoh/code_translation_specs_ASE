#include <stdio.h>

int main(void)
{
    int m, f, b;

    scanf("%d%d%d", &m, &f, &b);
    
    if (m >= b){
        printf("0\n");
    }
    else if (m + f >= b){
        printf("%d\n", (b - m));
    }
    else {
        printf("NA\n");
    }

    return (0);
}

