#include<stdio.h>
int main(){
  int n,x,y,h,w,s,s1,s2,a,i;
  int m[]={600,800,1000,1200,1400,1600,0};
  while(1){
    scanf("%d",&n);
    if(n==0)break;
    a=0;
    for(i=0;i<n;i++){
      scanf("%d %d %d %d",&x,&y,&h,&w);
      if     (x+y+h>160)s1=6;
      else if(x+y+h>140)s1=5;
      else if(x+y+h>120)s1=4;
      else if(x+y+h>100)s1=3;
      else if(x+y+h> 80)s1=2;
      else if(x+y+h> 60)s1=1;
      else              s1=0;
      if     (w>25)s2=6;
      else if(w>20)s2=5;
      else if(w>15)s2=4;
      else if(w>10)s2=3;
      else if(w>5 )s2=2;
      else if(w>2 )s2=1;
      else         s2=0;
      if(s1>s2)s=s1;
      else     s=s2;
      a+=m[s];
    }
    printf("%d\n",a);
  }
  return 0;
}