#include <stdio.h>

int main(){
    int i;
    int c[3];
    scanf("%d %d %d",&c[0],&c[1],&c[2]);
    for(i=0;i<10;i++){
        c[2] = c[0]*c[2]-c[1];
        printf("%d\n",c[2]);
    }
    return 0;
}