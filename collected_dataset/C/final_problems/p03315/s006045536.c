#include<stdio.h>
int main()
{
    char n[5];
    int s=0,i;
    scanf("%s",n);
    for(i=0;i<4;i++)
    {
        if(n[i]=='+') s++;
        else if(n[i]=='-') s--;
    }
    printf("%d\n",s);
}
