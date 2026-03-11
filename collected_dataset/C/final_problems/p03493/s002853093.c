#include <stdio.h>

int main(void) 
{
    char str[4]; int i, cnt = 0;
    scanf("%s", str);
    for(i = 0; i < 3; ++i) {
        if(str[i] == '1') { ++cnt; }
    }
    printf("%d\n", cnt);
    return 0;
}