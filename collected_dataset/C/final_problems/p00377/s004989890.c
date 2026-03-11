#include <stdio.h>

int main(void)
{
    int N, C;
    int p;
    int i;
    int sum, cake;
    
    scanf("%d%d", &N, &C);
    
    sum = 0;
    for (i = 0; i < C; i++){
        scanf("%d", &p);
        sum += p;
    }
    
    cake = sum / (N + 1);
    if ((sum % (N + 1)) > 0){
        cake++;
    }

    printf("%d\n", cake);

    return (0);
}

