#include <stdio.h>

int main() {

    int a, b;
    int sum = 0;
    int num = 1;

    scanf("%d %d", &a, &b);

    sum = a;

    for (; sum < b ;) {
        sum += (a - 1);
        num++;
    }
    
    if(b == 1) {
        num = 0;
    }

    printf("%d", num);
    return 0;
}