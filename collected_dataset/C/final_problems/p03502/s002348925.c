#include <stdio.h>
#include <math.h>

int main(void)
{
        int n,c=0,i,d;
        scanf("%d",&n);

        d=n;
        for(i=0;i<9;i++){
                c+=d/pow(10,9-i);
                d=d%(int)pow(10,9-i);
        }
        if(n%c==0)printf("Yes\n");
        else printf("No\n");
        return 0;
}
