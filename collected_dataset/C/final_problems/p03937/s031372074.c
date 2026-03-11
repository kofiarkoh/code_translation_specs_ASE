#include <stdio.h>
#include <stdint.h>

int main(void)
{
    uint32_t H, W;
    uint32_t i;
    uint32_t min = 0, max;
    uint32_t possible = 1;
    scanf("%u %u\n", &H, &W);
    max = W-1;

    for (i = 0; i < H; i++) {
        char c;
        uint32_t found = 0;
        uint32_t found_then_not_found = 0;
        uint32_t x = 0;
        while ((c = getchar()) != '\n') {
            if (!possible) continue;
            if (c == '#') {
                max = x;
            }
            if (found == 0 && c == '#') {
                found = 1;
                if (x < min) {
                    possible = 0;
                } 
            } else if (found_then_not_found == 1 && c == '#') {
                possible = 0;
            } else if (found == 1 && c == '.') {
                found_then_not_found = 1;
            }
            x++;
        }
        min = max;
    }

    if (possible) {
        printf("Possible\n");
    } else {
        printf("Impossible\n");
    }
    return 0;
}

