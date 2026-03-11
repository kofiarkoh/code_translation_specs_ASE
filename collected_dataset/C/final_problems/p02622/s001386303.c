//Atcoder Biginner Contest 172 B問題

#include <stdio.h>

#define LENGTH 200000

int main(void) {
    char n[LENGTH];
    char m[LENGTH];
    int count = 0;

    scanf("%s", n);
    scanf("%s", m);

    for (int i = 0; n[i] != '\0'; i++) {
        if (n[i] != m[i]) {
            count += 1;
        }
    }

    printf("%d\n", count);

    return 0;
}
