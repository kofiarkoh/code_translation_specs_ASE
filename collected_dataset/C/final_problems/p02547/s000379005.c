#include<stdio.h>
int main(void){
    int a,b,i,k,n,s;
    k = 0;
    s = 0;
    scanf("%d",&n);
    for(i=1;i<n+1;i++){
        scanf("%d",&a);
        scanf("%d",&b);
        if(a == b)
        k = k + 1;
        if(a != b)
        k = 0;
        if(k == 3)
        s = 100;
    }
    if(s == 100)
    printf("Yes\n");
    else
    {
        printf("No\n");
    }
        return 0;
    
    
}