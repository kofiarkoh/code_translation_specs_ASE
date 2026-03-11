#include <stdio.h>
int main(void)
{
    int a, b, i;

    scanf("%d%d", &a, &b);

    if(a<=0 && b>=0){
        printf("Zero\n");
        return 0;
    }

    if(a>0 && b>0) printf("Positive\n");
    else if(a%2==0) printf("Positive\n");
    else printf("Negative\n");
    
    return 0;
}
