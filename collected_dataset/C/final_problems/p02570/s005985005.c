#include <stdio.h>
int main(void){
    int   D,T,S;
    double N;
    scanf("%d %d %d",&D,&T,&S);
    N=(D/S);
    if(N<=T){
        printf("Yes");
    }
    else{
        printf("No");
    }
    return 0;
}