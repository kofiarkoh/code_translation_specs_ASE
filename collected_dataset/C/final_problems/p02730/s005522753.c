#include<stdio.h>
#include<string.h>

int main()
{
    char str[99];
    gets(str);
    int n=0;
    n=strlen(str);
    int i,j,x,y=0,z=0;
    x=(n-1)/2;
    for (i=0,j=n-1;i<n/2;i++,j--)
    {
        if (str[i]==str[j])
            z++;
        else
        {
            printf("No\n");
           break;
        }
    }
    if (z==(n/2))
    {
        for (i=0,j=x-1;i<x/2;i++,j--)
        {
            if (str[i]==str[j])
                y++;
            else
            {
                printf("No\n");
                break;
            }
        }
    }

    if (y==(x/2))
        printf("Yes\n");
    return 0;
}
