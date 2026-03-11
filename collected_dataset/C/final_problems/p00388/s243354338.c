#include <stdio.h>

int main(void)
{
    int H, A, B;
    int h, n;
    
    scanf("%d%d%d", &H, &A, &B);
    
    n = 0;
    for (h = A; h <= B; h++){
        if (H % h == 0){
            n++;
        }
    }
    
    printf("%d\n", n);

    return (0);
}

