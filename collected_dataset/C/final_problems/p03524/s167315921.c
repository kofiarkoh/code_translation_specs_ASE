#include <stdio.h>
#include <string.h>

int main()
{
    char s[100001];
    scanf("%s",s);

    int len = strlen(s),count[3] = {0},i;
    for (i = 0; i < len; i++)
    {
        count[s[i] - 'a']++;
    }

    int min = 0;
    int max = 0;
    for (i = 1; i < 3; i++)
    {
        if(count[min] > count[i])min = i;
        if(count[max] < count[i])max = i;
    }
    
    for (i = 0; i < 3; i++)
    {
        count[i] -= count[min];
    }
    if(count[max] == count[3 - min - max])printf("YES");
    else printf("NO");
}