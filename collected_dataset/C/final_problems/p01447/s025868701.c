#include <stdio.h>

int main(void)
{
    unsigned int N;
    int count;
    
    scanf("%d", &N);
    
    count = 0;
    
    while (N > 3){
        switch (N % 3){
          case 0:
            N /= 3;
            break;
          
          case 1:
          case 2:
            N = N / 3 + 1;
            break;
        }
//printf("<%d>", N);
        count++;
    }
    
    count++;
    
    printf("%d\n", count);
    
    return (0);
}