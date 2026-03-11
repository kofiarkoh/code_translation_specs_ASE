#include<stdio.h>
#include<string.h>

#define TMP (signed int)30
#define YES "Yes"
#define NO "No"

int main()
{
    signed int x = 0;

    scanf("%d", &x);

    if (x >= TMP)
        printf("%s", YES);
    else
        printf("%s", NO);

    return (0);
}