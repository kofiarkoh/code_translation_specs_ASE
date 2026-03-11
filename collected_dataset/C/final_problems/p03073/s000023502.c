#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    int i,n,sum1,sum2;
    char s[100001];
    gets(s);
    n=strlen(s);
    sum1=sum2=0;
    for(i=0; i<n; i++)
    {
        if(i%2==0&&s[i]=='0')
            sum1++;
        else if(i%2==1&&s[i]=='1')
            sum1++;
        if(i%2==0&&s[i]=='1')
            sum2++;
        else if(i%2==1&&s[i]=='0')
            sum2++;

    }
    if(sum1<sum2)
        printf("%d\n",sum1);
    else
        printf("%d\n",sum2);
    return 0;
}
