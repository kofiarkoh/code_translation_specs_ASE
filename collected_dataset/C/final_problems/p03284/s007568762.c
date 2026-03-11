#include <stdio.h>
int main()
{
    int N,K;
    scanf("%d %d",&N,&K);
    if(N>K){
        printf("%d",N%K);
    }else if(N==K){
        printf("%d",0);
    }else{
        printf("%d",1);
    }
    return 0;
}
