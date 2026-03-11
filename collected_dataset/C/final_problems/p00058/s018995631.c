#include <stdio.h>

int main(){
  double xa,ya,xb,yb,xc,yc,xd,yd,ans;
  while(scanf("%lf%lf%lf%lf%lf%lf%lf%lf",&xa,&ya,&xb,&yb,&xc,&yc,&xd,&yd)!=EOF){
    ans=(xd-xc)*(xb-xa)+(yd-yc)*(yb-ya);
    ans*=(ans<0?-1:1);
    if(ans<=1.0e-11 || 
       (xa==xb && yc==yd) || (xc==xd && ya==yb)) printf("YES\n");
    else printf("NO\n");
  }
  
  return 0;
}