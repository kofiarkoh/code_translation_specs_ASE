#include<stdio.h>
#include<string.h>
int main(void){
    int unused __attribute__((unused));
    char name[1000];
    unused = scanf("%s", name);
    int length = strlen(name) - 1;
    char l = name[length];
    if(l != 's'){
        name[length + 1] = 's';
        name[length + 2] = '\0';
    }
    else if(l == 's'){
        name[length + 1] = 'e';
        name[length + 2] = 's';
        name[length + 3] = '\0';
    }
    printf("%s", name);
    x:
    return 0;
}