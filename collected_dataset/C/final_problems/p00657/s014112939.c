#include <stdio.h>

int main(void){
    int r,c;
    for(;scanf("%d %d",&r,&c),(r+c)!=0;)
        printf("%s\n",r*c%2==0?"yes":"no");
    return 0;
}