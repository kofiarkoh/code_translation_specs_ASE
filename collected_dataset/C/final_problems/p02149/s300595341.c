#include <stdio.h>

int main(void)
{
    int a, b, c;
    
    scanf("%d%d%d", &a, &b, &c);
    
    if (a > b && a > c){
        printf("A\n");
        return (0);
    }
    
    if (b > c){
        printf("B\n");
        return (0);
    }
    printf("C\n");
    
    return (0);
}

        
