#include <stdio.h>
#include <string.h>

int main()
{
    char s[101];
    long long int k,i,ans = 0;

    scanf("%s %lld",s,&k);

    int len = strlen(s);

    for (i = 0; i < len; i++)
    {
        if (s[i] != '1')
        {
            ans = s[i] - '0';
            break;
        }
    }
  
    if (i == len)
    {
        ans = 1;
    }
    else if(i + 1 >= k)
    {
        ans = 1;
    }
    
    printf("%lld",ans);
}