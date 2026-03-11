#include<stdio.h>
#include<stdlib.h>
#include<string.h>
void fun(char *);
void fun(char *s)
{
    int len=strlen(s);
    if(s[0]==s[len-1])
    {
        if(len%2==0)
        {
            printf("First\n");
        }
        else
        {
            printf("Second\n");
        }
    }
    else
    {
        if(len%2==0)
        {
            printf("Second\n");
        }
        else
        {
            printf("First\n");
        }
    }
}
int main()
{
    char a[10010];
    gets(a);
    fun(a);
    return 0;
}