#include <stdio.h>
#include <string.h>

int main(void)
{
    int a, b;
    char s[4];
    scanf("%s", s);

    a = (s[0] - '0') * 10 + (s[1] - '0');
    b = (s[2] - '0') * 10 + (s[3] - '0');

    if(a <= 12 && b <= 12){
        printf("%s\n", "AMBIGUOUS");
        return 0;
    }

    if(a == 00 || b == 00){
        printf("%s\n", "NA");
        return 0;
    }

    if(a >= 01 && b <= 12){
        printf("%s\n", "YYMM");
        return 0;
    }

    if(a <= 12 && b >= 01){
        printf("%s\n", "MMYY");
        return 0;
    }

}