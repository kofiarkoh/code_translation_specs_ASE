#include <stdio.h>

int main(void)
{
    int n;
    int i;

    while (1){
        scanf("%d", &n);
        if (n == 0){
            break;
        }

        for (i = 0; n != 1; i++){
            if (n % 2 == 0){
                n /= 2;
            }
            else {
                n = n * 3 + 1;
            }
        }
        printf("%d\n", i);
    }

    return (0);
}