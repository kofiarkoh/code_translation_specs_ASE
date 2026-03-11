#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int main() {
    char* inputStr = malloc(1000000);
    long K;
    scanf("%s", inputStr);
    scanf("%ld", &K);
    for (long i=0;i<strlen(inputStr);i++) {
        if (K <= 0) {
            break;
        }
        if (inputStr[i] == 'a') {
            if (K >=26) {
                K -= 26;
                continue;
            }
        }
        int need = 122 - inputStr[i] + 1;
        if (need <= K) {
            inputStr[i] = 'a';
            K -= need;
        }
    }
    if (K != 0) {
        inputStr[strlen(inputStr)-1] = ((inputStr[strlen(inputStr)-1] + K) % 97) + 97;
    }
    printf("%s\n", inputStr);
}