#include<stdio.h>

int main(void) {

    int W,H,x,y,r,flg=0;
    scanf("%d%d%d%d%d",&W,&H,&x,&y,&r);

    if ((x-r)>=0 && (x+r)<=W) flg++;
    if ((y-r)>=0 && (y+r)<=H) flg++;

    if (flg==2) {
        printf("Yes\n");
    } else printf("No\n");

    return 0;
}