/*
 * ID: ITP1_8_B
 * Problem: Sums of numbers
 */

#include <stdio.h>
#define MAX_DIGIT 1000

int main(void)
{
    while (1) {
        char number[MAX_DIGIT + 2];
        fgets(number, sizeof(number), stdin);
        
        if (number[0] == '0')
            return 0;

        int sum = 0;
        int i;
        for (i = 0; i < MAX_DIGIT; i++) {
            if (number[i] == '\n')
                break;
            sum += number[i] - '0';
        }

        printf("%d\n", sum);
    }

    return 0;
}

