#include<stdio.h>
int main(){
  int a,b,c,d,e,f,g,h;
  double m,n;
  a=b=c=d=e=f=g=h=0;
  while(scanf("%lf%lf",&m,&n)!=-1){
    if(0.2>m)
      d++;
    else if(0.6>m)
      c++;
    else if(1.1>m)
      b++;
    else
      a++;
    if(0.2>n)
      h++;
    else if(0.6>n)
      g++;
    else if(1.1>n)
      f++;
    else
      e++;
  }
  printf("%d %d\n",a,e);
  printf("%d %d\n",b,f);
  printf("%d %d\n",c,g);
  printf("%d %d\n",d,h);
  return 0;
}