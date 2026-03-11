#include <stdio.h>
#include <string.h>

int main(int argc, const char * argv[])
{

    char str[110];
    char before[110];
    char after[110];
    long len;
    long halflen;
    int cnt = 0;
    long i;
    
    while (scanf("%s", str) != EOF) {
        len = strlen(str);
        halflen = len / 2;
        strncpy(before, str, halflen);
        if (len == 1) cnt++;
        else if (len % 2 == 0) {
            for (i = len - 1; halflen <= i; i--) after[len - i - 1] = str[i];
            if (len == 1 || strcmp(before, after) == 0) cnt++;
        }
        else {
            for (i = len - 1; halflen + 1 <= i; i--) after[len - i - 1] = str[i];
            if (strcmp(before, after) == 0) cnt++;
        }
    }
    printf("%d\n", cnt);
    return 0;
}