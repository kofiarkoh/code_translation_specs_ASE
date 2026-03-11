#include<stdio.h>
int main()
{
    int a,b,i;
    scanf("%d%d",&a,&b);
    char k[20];
    if(a==b)
    {
        for(i=0; i<a; i++)
        {
            k[i]=a;
        }
        k[i]='\0';
        for(i=0; k[i]!='\0'; i++)
        {
            printf("%d",k[i]);
        }
    }
    else if(a>b)
    {
        for(i=0; i<a; i++)
        {
            k[i]=b;
        }
        k[i]='\0';
        for(i=0; k[i]!='\0'; i++)
        {
            printf("%d",k[i]);
        }
    }
    else
    {
        for(i=0; i<b; i++)
        {
            k[i]=a;
        }
        k[i]='\0';
        for(i=0; k[i]!='\0'; i++)
        {
            printf("%d",k[i]);
        }
    }
    return 0;
}
