
#include <stdio.h>

int main()
{   int a,b;
    scanf("%d%d",&a,&b);
    if(a>b){
        int tmp=b;
        b=a;
        a=tmp;
    }
    if((b-a)%2==0){
    printf("%d",b-(b-a)/2);
    }else{
        printf("IMPOSSIBLE");
    }
    return 0;
}
