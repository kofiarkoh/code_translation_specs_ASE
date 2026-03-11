#include<stdio.h>
#include<string.h>

int main()
{
    int k, n;
    char s[1000];

    scanf("%d %s %d", &n, s, &k);

    for(int i=0; i<n; i++)
    {
        if(s[i]!=s[k-1]) printf("*");
        else printf("%c", s[k-1]);
    }
    printf("\n");
    return 0;
}
