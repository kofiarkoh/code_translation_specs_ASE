#include<stdio.h>
#include<math.h>
int main(){
  int n,i;
  double x1,x2,r1,y1,y2,r2;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%lf %lf %lf %lf %lf %lf",&x1,&y1,&r1,&x2,&y2,&r2);
    if(sqrt((x1-x2)*(x1-x2)+(y1-y2)*(y1-y2))+r2<r1)printf("2\n");
    else if(sqrt((x1-x2)*(x1-x2)+(y1-y2)*(y1-y2))+r1<r2)printf("-2\n");
    else if(sqrt((x1-x2)*(x1-x2)+(y1-y2)*(y1-y2))<=r2+r1)printf("1\n");
    else printf("0\n");
  }
  return 0;
}