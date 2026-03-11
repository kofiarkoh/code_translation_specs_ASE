#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#define CHK(ch) ((ch == 'A' || ch == 'C' || ch == 'G' || ch == 'T') ? true : false)

int main(void)
{
    char str[12]; scanf("%s", str);
    int len1 = 0, len2 = 0, n = strlen(str), i;
    bool pos = false;
    for(i = 0; i < n; ++i) {
        if(pos) {
            if(CHK(str[i])) { ++len1; }
            else { break; }
        }
        else {
            if(CHK(str[i])) { ++len1; pos = true; }
        }
    }
    
    if(i < n) {
        pos = false;
        for( ; i < n; ++i) {
            if(pos) {
                if(CHK(str[i])) { ++len2; }
                else { len1 = ((len1 < len2) ? len2 : len1 ); pos = false; len2 = 0;}
            }
            else {
                if(CHK(str[i])) { ++len2; pos = true; }
            }
        }
    }
    
    //printf("len1 : %d len2 : %d i : %d\n", len1, len2, i);
    printf("%d\n", (len1 < len2) ? len2 : len1);
    return 0;
}