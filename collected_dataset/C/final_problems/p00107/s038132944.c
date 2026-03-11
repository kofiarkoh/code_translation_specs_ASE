#include<stdio.h>
int main(){
  int i,x,y,z,r,n;
  double en;
  while(1){

  scanf("%d%d%d",&x,&y,&z);
  if(x==0&&y==0&&z==0) break;
 scanf("%d",&n);

 for(i=0;i<n;i++){
   scanf("%d",&r);
   en=(double)r*r*3.1415;
   if(x*y<=en && x*z<=en && y*z<=en) printf("OK\n");
   else printf("NA\n");
 }
  }
 return 0;
}