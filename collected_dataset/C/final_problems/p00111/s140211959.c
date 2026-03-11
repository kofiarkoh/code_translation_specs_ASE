#include <stdio.h>
#include <string.h>

char *enc[] = {
    "00000", "00001", "00010", "00011",
    "00100", "00101", "00110", "00111",
    "01000", "01001", "01010", "01011",
    "01100", "01101", "01110", "01111",
    "10000", "10001", "10010", "10011",
    "10100", "10101", "10110", "10111",
    "11000", "11001", "11010", "11011",
    "11100", "11101", "11110", "11111",
};

char *dec[] = {
    "100101", "10011010", "0101", "0001",
    "110", "01001", "10011011", "010000",
    "0111", "10011000", "0110", "00100",
    "10011001", "10011110", "00101", "111",
    "10011111", "1000", "00110", "00111",
    "10011100", "10011101", "000010", "10010010",
    "10010011", "10010000", "101", "000000",
    "000011", "10010001", "010001", "000001",
};

char str[2048];
char changed[2048];

int check(char c)
{
    if ('A' <= c && c <= 'Z') return 0;
    if (c == '0' || c == '1') return 0;
    if (c == ' ' || c == '\'') return 0;
    if (c == ',' || c == '-') return 0;
    if (c == '.' || c == '?') return 0;
    return -1;
}

int main(void)
{
    while (fgets(str, 2040, stdin) != NULL){
        int i, j;

        memset(changed, 0, sizeof(changed));
        for (i = j = 0; check(str[i]) == 0; i++, j += 5){
            int e;

            switch (str[i]){
                case ' ': e = 26; break;
                case '.': e = 27; break;
                case ',': e = 28; break;
                case '-': e = 29; break;
                case '\'': e = 30; break;
                case '?': e = 31; break;
                default: e = str[i] - 'A'; break;
            }
            sprintf(changed + j, "%s", enc[e]);
        }
        while (j % 5 != 0) changed[j++] = '0';
        for (i = 0; check(changed[i]) == 0; ){
            for (j = 0; j < 32; j++){
                int len = strlen(dec[j]);
                if (strncmp(changed + i, dec[j], len) == 0){
                    switch (j){
                        case 26: putchar(' '); break;
                        case 27: putchar('\''); break;
                        case 28: putchar(','); break;
                        case 29: putchar('-'); break;
                        case 30: putchar('.'); break;
                        case 31: putchar('?'); break;
                        default: putchar('A' + j); break;
                    }
                    i += len;
                    break;
                }
            }
            if (j == 32){
                puts("");
                break;
            }
        }
    }

    return 0;
}