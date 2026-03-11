#include <stdio.h>
#include <stdlib.h>

#define STRINGSIZE 17

int main (){
    int noCharacters = 0;
    char string[STRINGSIZE];
    char comparedTo[STRINGSIZE] = "CODEFESTIVAL2016";
    scanf("%s",string);
    int i;
    for (i=0;i<STRINGSIZE;i++) {
        if (string[i] != comparedTo[i]) {
            noCharacters++;
        }
    }
    if(string[3] == string[5]) {
        if (string[3] != 'E'){
            noCharacters--;
        }

    }
    printf("%d",noCharacters);
    return EXIT_SUCCESS;
}