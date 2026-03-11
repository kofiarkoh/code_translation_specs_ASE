#include <stdio.h>

int main() {
    int cnt = 0;

    char S[6];
    for (int i = 0; i < 6; i++) {
        scanf("%c", &S[i]);
    }

    for (int i = 0; i < 6; i++) {
        if (S[i] == '1') {
            cnt++;
        }
    }

    printf("%d\n", cnt);
    return 0;
}