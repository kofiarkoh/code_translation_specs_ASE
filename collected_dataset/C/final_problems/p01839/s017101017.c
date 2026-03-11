#include <stdio.h>

int main(void){
    int n,t=0,i,sign=0;
    char mozi[3];
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%s",mozi);
        if(mozi[0] == 'A')
            t++;
        else
            t--;
        if(t == -1)
            sign=1;
    }
    if(sign == 1 ||  t != 0)
        printf("NO\n");
    else
        printf("YES\n");
    return 0;
}