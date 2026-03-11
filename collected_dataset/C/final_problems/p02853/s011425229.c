#include <stdio.h>

int main()
{   
    int x,y,z=0;
    int t=300000;
    int s=200000;
    int q=100000;
    int r=400000;
    scanf("%d", &x);
    scanf("%d", &y);
    if(x==1 && y==1)
    {
        z=t+t+r;
        //printf("%d",z);
    }
    else if(x==2 && y==2)
    {
        z=s+s;
        //printf("%d",z);
    }
    else if(x==3 && y==3)
    {
        z=q+q;
        //printf("%d",z);
    }
     else if((x==1 && y==2) || (x==2 && y==1))
    {
        z=s+t;
    }
       else if((x==1 && y==3) || (x==3 && y==1))
    {
        z=q+t;
    }
       else if((x==3 && y==2) || (x==2 && y==3))
    {
        z=s+q;
    }
    else if(x==1 || y==1)
    {
        z=t;
    }
     else if(x==2 || y==2)
    {
        z=s;
    }
     else if(x==3 || y==3)
    {
        z=q;
    }
     printf("%d",z);
    return 0;    
}
