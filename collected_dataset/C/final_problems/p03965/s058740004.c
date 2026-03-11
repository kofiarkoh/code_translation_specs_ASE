#include<stdio.h>
#include<stdlib.h>
int fun(char *);
int fun(char *s)
{
    int p=1,ans=0;
    int i;
    if(s[0]=='g')
    {
        ans=0;
    }
    if(s[0]=='p')
    {
        ans=-1;
    }
    for(i=1;s[i]!='\0';i++)
    {
        if(s[i]=='g')
        {
            if(p>0)
            {
                ans++;
                p--;
            }
            else
            {
                p++;
            }
        }
        if(s[i]=='p')
        {
            if(p>0)
            {
                p--;
            }
            else
            {
                ans--;
                p++;
            }
        }
    }
    return ans;
}
int main()
{
    char a[100010];
    scanf("%s",a);
    printf("%d\n",fun(a));
    return 0;
}