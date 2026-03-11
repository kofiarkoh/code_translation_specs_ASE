#include <stdio.h>


int main (void) {
    int c, len = 0, first_print = 1;
    char word[7];

    while (1) {
        c = getc(stdin);
        if (('0' <= c && c <= '9') ||
            ('A' <= c && c <= 'Z') ||
            ('a' <= c && c <= 'z')) {
            if (len < 6) {
                word[len] = c;
            }
            len++;
        }
        else {
            if (c == ' ' || c == '.' || c == ',' || c == '\n' || c == EOF) {
                if (2 < len && len < 7) {
                    word[len] = '\0';
                    printf("%s%s", first_print? "": " ", word);
                    first_print = 0;
                }
                if (c == EOF) break;
            }
            len = 0;
        }
    }
    printf("\n");
    return 0;
}