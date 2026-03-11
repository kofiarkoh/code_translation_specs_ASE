#include<stdio.h>
//#include<math.h>
int ifcl(int p0[2],int p1[2],int p2[2],int p3[2]){
  long long x0=p1[0]-p0[0],x1=p2[0]-p0[0],x2=p3[0]-p0[0]/*,x3=x2-x1,x4=x0-x1,x5=-x1*/;
  long long y0=p1[1]-p0[1],y1=p2[1]-p0[1],y2=p3[1]-p0[1]/*,y3=y2-y1,y4=y0-y1,y5=-y1*/;
  long long x3=p3[0]-p2[0],x4=p1[0]-p2[0],x5=p0[0]-p2[0];
  long long y3=p3[1]-p2[1],y4=p1[1]-p2[1],y5=p0[1]-p2[1];
  return ((x0*y1-y0*x1)*(x0*y2-y0*x2)<=0&&(x3*y4-y3*x4)*(x3*y5-y3*x5)<=0)?1:0;
}
/*double dMIN(double a,double b){return a<b?a:b;}
double drpl(int p0[2],int p1[2],int p2[2]){
  int a=p0[1]-p1[1],b=p0[0]-p1[0],c=p0[0]*p1[1]-p1[0]*p0[1];
  int d=p2[1]-p1[1],e=p2[0]-p1[0];
  int f=p2[1]-p0[1],g=p2[0]-p0[0];
  if(a*d+b*e<0||a*f+b*g>0)return dMIN(hypot(d,e),hypot(f,g));
  return abs(p2[0]*a-p2[1]*b+c)/hypot(a,b);
}
double drll(int *p0,int *p1,int *p2,int *p3){
  double l=drpl(p2,p3,p0);
  l=dMIN(l,drpl(p2,p3,p1));
  l=dMIN(l,drpl(p0,p1,p2));
  l=dMIN(l,drpl(p0,p1,p3));
  if(ifcl(p0,p1,p2,p3))l=0;
  return l;
  }*/
int main(){
  int p[4][2],n,i;
  scanf("%d",&n);
  while(n--){
    for(i=0;i<8;i++)scanf("%d",&p[i/2][i%2]);
    printf("%d\n",ifcl(p[0],p[1],p[2],p[3]));
  }
  return 0;
}