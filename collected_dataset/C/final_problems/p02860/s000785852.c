#include<stdio.h>
#include<string.h>
int main()
{
    int i,n;
    scanf("%d", &n);
    char s[n+1],t[n+2];
    scanf("%s", s);
        for(i=0;i<n/2;i++)
        {
            t[i]=s[i];
        }
        t[i]='\0';
        strcat(t,t);
        if(strcmp(t,s)==0)printf("Yes\n");
        else printf("No\n");
    return 0;
}
