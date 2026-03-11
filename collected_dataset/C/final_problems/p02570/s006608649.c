#include <stdio.h>

int main(void) {
    int D, T, S;
    double result;

    scanf("%d %d %d", &D, &T, &S);

    result = D/S;

    if(result > T) {
        puts("No");
    }
    else puts("Yes");

    return 0;
}