#include <stdio.h>
#include <string.h>

int main() {
    char S[10];
    scanf("%s", S);
    if (!strcmp(S, "Sunny")) {
          printf("%s\n", "Cloudy");
    } else if(!strcmp(S, "Cloudy")) {
          printf("%s\n", "Rainy");
    } else {
          printf("%s\n", "Sunny");
    }
    return 0;
}
