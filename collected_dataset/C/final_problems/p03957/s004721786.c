#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int main(void) {
    char input[100];
    bool f = 0;
    scanf("%s", input);
    for (int i = 0; i < 100; i++) {
        if (input[i] == 'C') {
            f = true;
        }
        if (f && input[i] == 'F') {
            printf("Yes\n");
            return 0;
        }
    }
    printf("No\n");
    return 0;
}
