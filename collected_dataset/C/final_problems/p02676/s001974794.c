#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main()
{
    int k;
    char s[105];
    scanf("%d",&k);
    scanf("%s",s);
    int l=strlen(s);
    if(l<=k)
    puts(s);
    else
    {
        for(int i=0;i<k;i++)
        printf("%c",s[i]);
        printf("...");
    }
}