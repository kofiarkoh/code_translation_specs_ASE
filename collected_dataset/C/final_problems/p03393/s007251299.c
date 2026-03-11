#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int main()
{
    char ch[27];
    int i,k,c,n;
    scanf("%s",ch);
    n=strlen(ch);
    if(n!=26)
    {
        k=97;
        for(i=0;i<n;i++)
        {
            if(ch[i]==k) 
                k++;
        }
        ch[n]=k;
        ch[n+1]='\0';
    }
    else
    {
        c=ch[n-1];
        for(i=n-1;i>0;i--)
        {
            if(ch[i]-ch[i-1]!=(-1))
            break;
        }
        if(i==0)
        {
            printf("-1");
            exit(0);
        }
        else
        {   ch[i-1]=c;
            ch[i]='\0';
        }
    }
    printf("%s",ch);
}
  