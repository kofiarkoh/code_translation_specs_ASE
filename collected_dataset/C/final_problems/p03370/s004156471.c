#include <stdio.h>


int main()
{
    int n,i,x,m[100],min=1000,nsum=0,msum=0;
    scanf("%d %d",&n,&x);
    
    for(i=0;i<n;i++){
        scanf("%d",&m[i]);
        if(m[i]<min){min=m[i];}
        nsum+=m[i];
    }
    x-=nsum;
    msum=n+x/min;
    printf("%d\n",msum);
    
    

    return 0;
}