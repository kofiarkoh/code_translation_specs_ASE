#include<stdio.h>
#include<math.h>
double f(double a,double b, double c){
  double s=(a+b+c)/2;
  return sqrt(s*(s-a)*(s-b)*(s-c));
}
int main(){
  double x1,y1,x2,y2,x3,y3,xp,yp,s,S,a,b,c,ap,bp,cp;
  while(scanf("%lf %lf %lf %lf %lf %lf %lf %lf",
	      &x1,&y1,&x2,&y2,&x3,&y3,&xp,&yp)!=EOF){
    a=hypot(x1-x2,y1-y2);
    b=hypot(x2-x3,y2-y3);
    c=hypot(x1-x3,y1-y3);
    ap=hypot(x1-xp,y1-yp);
    bp=hypot(x2-xp,y2-yp);
    cp=hypot(xp-x3,yp-y3);
    s =f(a,ap,bp);
    s+=f(b,bp,cp);
    s+=f(c,cp,ap);
    S =f(a,b,c);
    if(fabs(s-S)<0.00000001)printf("YES\n");
    else    printf("NO\n");
  }
  return 0;
}