#include <stdio.h>
int main() {
    int A, B, C, D;
    scanf("%d %d %d %d", &A, &B, &C, &D);
    while (1) {
        C=C-B;
        if(C<1) {
            printf("Yes\n");
            break;
        }
        A=A-D;
        if(A<1) {
            printf("No\n");
            break;
        }
    }
    return 0;
}