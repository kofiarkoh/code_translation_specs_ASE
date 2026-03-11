#include <stdio.h>

int main() {
    int M[2], D[2];

    for (int i = 0; i < 2; i++) {
        scanf("%d %d", &M[i], &D[i]);
    }

    if (M[0] != M[1]) {
        printf("%d\n", 1);
        return 0;
    } else {
        printf("%d\n", 0);
        return 0;
    }
}