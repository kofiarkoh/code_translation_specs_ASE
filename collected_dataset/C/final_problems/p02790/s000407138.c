#include <stdio.h>

int main(void){
    int a, b;
    scanf("%d", &a);
    scanf("%d", &b);
    for(int i=0; i<(a>b ? a: b); ++i){
        printf("%d", a>b ? b: a);
    }
    printf("\n");
    return 0;
}
