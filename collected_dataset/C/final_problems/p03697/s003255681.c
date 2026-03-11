#include <stdio.h>

int main()
{
    int a, b, sum;

    scanf("%d%d", &a, &b);
    char str[] = "error";

    sum = a + b;

    if(sum >= 10){
        printf("%s\n", str);
    }

    else{
        printf("%d\n", sum);
    }


    return 0;
}

