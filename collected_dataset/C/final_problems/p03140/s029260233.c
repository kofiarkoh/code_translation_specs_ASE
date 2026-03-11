#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    int n,count = 0,ans = 0;
    char a[101],b[101],c[101];
    scanf("%d %s %s %s",&n,a,b,c);

    for (int i = 0; i < n; i++)
    {
        count = 0;
        if(a[i] == b[i])count++;
        if(b[i] == c[i])count++;
        if(c[i] == a[i])count++;

        if(count == 0)ans += 2;
        if(count == 1)ans += 1;
    }
    printf("%d",ans);
}