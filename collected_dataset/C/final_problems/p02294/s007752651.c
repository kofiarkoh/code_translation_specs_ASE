#include<stdio.h>
#define max(a,b) a>b?a:b
#define min(a,b) a<b?a:b
int main(){
    int n,ax,ay,bx,by,cx,cy,dx,dy;
    double x,y,t;
    scanf("%d",&n);
    while(n--){
        scanf("%d%d%d%d%d%d%d%d",&ax,&ay,&bx,&by,&cx,&cy,&dx,&dy);
        t=(double)((cx-ax)*(dy-cy)-(cy-ay)*(dx-cx))/(double)((bx-ax)*(dy-cy)-(by-ay)*(dx-cx));
        x=ax+t*(bx-ax);
        y=ay+t*(by-ay);
        if((double)(min(ax,bx))<=x&&x<=(double)(max(ax,bx))&&(double)(min(ay,by))<=y&&y<=(double)(max(ay,by))){
            if((double)(min(cx,dx))<=x&&x<=(double)(max(cx,dx))&&(double)(min(cy,dy))<=y&&y<=(double)(max(cy,dy)))printf("%d\n",1);
        }else printf("%d\n",0);
    }
    return 0;
}