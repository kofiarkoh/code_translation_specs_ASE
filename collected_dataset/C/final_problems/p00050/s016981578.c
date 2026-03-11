#include<stdio.h>
#include<string.h>

int main(){
    int i, len;
    char str[1001];

    while(fgets(str, 1000, stdin) != NULL){
        len = strlen(str);
        for(i = 0; i < len - 5; i++){
            if(strncmp(&str[i], "apple", 5) == 0) strncpy(&str[i], "peach", 5);
            else if(strncmp(&str[i], "peach", 5) == 0) strncpy(&str[i], "apple", 5);
        }
        printf("%s", str);
    }
    return 0;
}