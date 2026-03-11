#include <stdio.h>
#include <string.h>
#include <ctype.h>

int main(){
    int i;
    int a = 0;
    int b = 0;
    int ab = 0;
    int o = 0;
    char str[10];
    while(1){
        if(scanf("%d,%s",&i,str) == EOF)break;
        if(str[0] == 'A'){
            if(str[1] == 'B'){
                ab = ab + 1;
            }
            else{
                a = a + 1;
            }
        }
        else if(str[0] == 'B'){
            b = b + 1;
        }
        else if(str[0] == 'O'){
            o = o + 1;
        }
    }
    printf("%d\n",a);
    printf("%d\n",b);
    printf("%d\n",ab);
    printf("%d\n",o);
    return 0;
}