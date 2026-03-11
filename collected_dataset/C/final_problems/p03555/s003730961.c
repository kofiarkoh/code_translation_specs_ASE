#include<stdio.h>
int main()
{
    char a[4][4];
    char b[4][4];
    int sum=0;
    for(int i=0; i<2; i++)
    {
        scanf("%s",&a[i]);
    }
    if(a[0][0]==a[1][2])
        sum++;
    if(a[0][1]==a[1][1])
        sum++;
    if(a[0][2]==a[1][0])
        sum++;
    if(sum==3)
    {
        printf("YES\n");
    }
    else
    {
        printf("NO\n");
    }
    return 0;
}
