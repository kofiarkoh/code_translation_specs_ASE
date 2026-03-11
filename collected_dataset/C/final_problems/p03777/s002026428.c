#include <stdio.h>

int main() {
    char a, b;
    scanf("%c %c", &a, &b);

    char c;
    if (a == 'H') {
        if (b == 'H') {
            c = 'H';
        } else {
            c = 'D';
        }
    } else { // D
        if (b == 'H') {
            c = 'D';
        } else {
            c = 'H';
        }
    }

    printf("%c\n", c);
    return 0;
}