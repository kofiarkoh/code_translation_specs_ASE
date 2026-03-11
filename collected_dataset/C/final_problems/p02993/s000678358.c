#include <stdio.h>

int main () {
    char S[4];
    scanf("%s", S);
    char prev = 0;
    int i = 0;
    while(S[i]) {
        if (prev == S[i]) {
            printf("Bad\n");
            return 0;
        }
        prev = S[i];
        i++;
    }
    printf("Good\n");
    return 0;
}
