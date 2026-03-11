#include <stdio.h>


int main() {
    char tk;
    scanf("%c", &tk);
    for(char i = 'a'; i < 'z'; i++){
        if(tk == i)
            printf("%c", i+1);
    }
    return 0;
}
