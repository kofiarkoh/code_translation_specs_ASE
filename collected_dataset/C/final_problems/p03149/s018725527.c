#include <stdio.h>
int main(void){
    int a,b,c,d;
    scanf("%d%d%d%d",&a,&b,&c,&d);
    
    if((a-b)*(a-c)*(a-d)*(b-c)*(b-d)*(c-d))
    {
        if(a==1 || a==4 || a==7 || a==9)
        {
        if(b==1 || b==4 || b==7 || b==9)
        {
        if(c==1 || c==4 || c==7 || c==9)
        {
        if(d==1 || d==4 || d==7 || d==9)
        {
            printf("YES");
            return 0;
        }
        }
        }
        }
    }
    printf("NO");
    return 0;
}
