#include<stdio.h>
int main()
{
    int H,W,h,w;
    scanf("%d %d %d %d",&H,&W,&h,&w);
    H=(H-h)*(W-w);
    printf("%d",H);
    return 0;
}
