#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int n,hx,hy,dx,dy;
int cx[20],cy[20],flag[20];
int i,j;

int check(int c,double rsum,int hx,int hy){
    double r;
    if(c==n)return 1;
    for(i=0;i<n;i++){
        if(flag[i]==0){
            r=rsum+sqrt((hx-cx[i])*(hx-cx[i])+(hy-cy[i])*(hy-cy[i]));
            if(r+1e-8>sqrt((dx-cx[i])*(dx-cx[i])+(dy-cy[i])*(dy-cy[i])))return 0;
        }
    }
    for(i=0;i<n;i++){
        if(flag[i]==0){
            flag[i]=1;
            r=rsum+sqrt((hx-cx[i])*(hx-cx[i])+(hy-cy[i])*(hy-cy[i]));
            if(check(c+1,r,cx[i],cy[i])==0)flag[i]=0;
            else return 2;
        }
    }
    return 0;
}

int main(void){
    while(1){
        scanf("%d %d %d %d %d",&n,&hx,&hy,&dx,&dy);
        if(n==0 && hx==0 && hy==0 && dx==0 && dy==0)break;
        for(i=0;i<n;i++){
            scanf("%d %d",&cx[i],&cy[i]);
            flag[i]=0;
        }
        if(check(0,0.0,hx,hy)==2)printf("YES\n");
        else printf("NO\n");
    }
    return 0;
}