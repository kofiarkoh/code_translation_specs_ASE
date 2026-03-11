#include<stdio.h>
#include<string.h>
int main()
{
    int i,n;
    char s[1000];
    while(scanf("%s",s)==1)
    {
        //printf("%s",s);
        n=strlen(s);
       // for(i=1;i<n;i++)
            //if(s[i]==' ')
        if(s[n-1]!='s')
        {
           s[n]='s';
           s[n+1]='\0';
           printf("%s\n",s);
        }
        else
        {
           s[n]='e';
           s[n+1]='s';
           s[n+2]='\0';
           printf("%s\n",s);
        }
           // printf("%c\n",s[n-1]);
    }
    return 0;
}
