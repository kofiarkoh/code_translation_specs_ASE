#include<stdio.h>

int n,nn[100],x,y,z;

int main()
{
    for(x=0;x<=25;x++){
        for(y=0;y<=14;y++){
            if(x*4+y*7<=100){
                nn[x*4+y*7]=1;
            }
        }
    }

    scanf("%d",&n);
    if(nn[n])printf("Yes\n");
    else printf("No\n");

    return 0;
}
