#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main()
{
    char s[100010];
    scanf("%s", s);
    int len = strlen(s);
    int p[27];
    memset(p, 0, sizeof(p));
    int i;
    for(i = 0; i < len; i++)
    {
        p[s[i]-'a']++;
    }
    int num = 0;
    for(i = 0; i < 26; i++)
    {
        if(p[i] > 1)
        {
            num++;
        }
    }
    if(num == 0)
    {
        if(len % 2 == 1)
        {
            printf("First\n");
        }
        else printf("Second\n");
    }
    else
    {
        if(s[0] == s[len - 1])
        {
            if(len % 2 == 1)
            {
                printf("Second\n");
            }
            else printf("First\n");
        }
        else
        {
            if(len % 2 == 1)
            {
                printf("First\n");
            }
            else printf("Second\n");
        }
    }
    return 0;
}