#include <stdio.h>

int main() {

    int a, b, tmp;
    
    scanf("%d %d", &a, &b);
    
    tmp = a * b;
    
    if (tmp < a - b) {
        tmp = a - b;
    } else if(tmp < a + b) {
        tmp = a + b;
    }
    
    printf("%d\n", tmp);
    
    return 0;
}