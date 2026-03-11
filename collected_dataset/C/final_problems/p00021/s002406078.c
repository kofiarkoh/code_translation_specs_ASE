#include <stdio.h>

int main(){
  int a,b;
  float c,d,e;
  float x1,y1,x2,y2,x3,y3,x4,y4;
  scanf("%d",&a);
  for(b=0;b<a;b++){
    scanf("%f %f %f %f %f %f %f %f",&x1,&y1,&x2,&y2,&x3,&y3,&x4,&y4);
    if((x2-x1)==.0) c=1.0;
      else c=(y2-y1)/(x2-x1);
    if((x3-x4)==.0) d=1.0;
      else d=(y3-y4)/(x3-x4);
    e=(c-d);e=(e<0)?-e:e;
    if(e<1e-9) printf("YES\n");
    else printf("NO\n");
  }
  return 0;
}