#include<stdio.h>
int gcm(int p,int q)
{
 int r;
 r=p%q;
 while(r!=0){
  p=q;
  q=r;
  r=p%q;
 }
 return q;
}

int main(void)
{
 int a,b,c,d,h,w;
 scanf("%d%d%d",&w,&h,&c);
 if(w>h){
  a=gcm(w,h);
 }
 else if(w==h){
  a=w;
 }
 else{
  a=gcm(h,w);
 }
 b=w/a;
 d=h/a;
 printf("%d",b*c*d);
 return(0);
}