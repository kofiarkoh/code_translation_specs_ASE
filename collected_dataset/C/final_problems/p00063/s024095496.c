#include <stdio.h>
#include <string.h>

int is_odd_Symmetry(const char str[], int len);
int is_even_Symmetry(const char str[], int len);

int main(void) {
    char str[100];
    int cnt = 0;
    int len;
    
    while(scanf("%s", str) != EOF) {
        len = (int)strlen(str);
        
        if(len == 1) {
            cnt++;
        } else {
            if(len % 2 != 0) {
                if(is_odd_Symmetry(str, len)) {
                    cnt++;
                }
            } else if(len % 2 == 0) {
                if(is_even_Symmetry(str, len)) {
                    cnt++;
                }
            }
        }
    }
    
    printf("%d\n", cnt);
    
    return(0);
}

int is_odd_Symmetry(const char str[], int len) {
    int i, range;
    int count = 0;
    
    range = len / 2;
    
    for(i = 0; i < range; i++) {
        if(str[i] == str[len - i - 1]) {
            count++;
        }
    }
    
    if(count == range) {
        return(1);
    }
    else {
        return(0);
    }
}

int is_even_Symmetry(const char str[], int len) {
    int i, range;
    int count = 0;
    
    range = (len / 2);
    
    for(i = 0; i < range; i++) {
        if(str[i] == str[len - i - 1]) {
            count++;
        }
    }
    
    if(count == range)
        return(1);
    else
        return(0);
}