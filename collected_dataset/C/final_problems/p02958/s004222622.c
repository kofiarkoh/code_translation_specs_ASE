#include <stdio.h>
int main(void){
    int c,i,n,p[50] = {};
    c = 0;
    scanf("%d",&n);
    for (i = 0;i < 50;i++)
    {
        scanf("%d",&p[i]);
        if (i == n)
        break;
    }
    for (i=2;i < 50;i++)
    {
    if (p[i]-p[i-1] != 1)
    {
        c = c+1;
    }
    if (i == n)
    {
        break;
    }
    }
if (c < n-1)
{
    printf("YES");
}
else{printf("NO");
}
return 0;
}