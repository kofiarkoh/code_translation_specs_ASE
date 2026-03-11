#include<stdio.h>
int main()
{
  int  h,hh, w,ww,max,i,x,xx,j,a1,a2;
  while(1){
    scanf("%d%d",&h,&w);
    if(h==0&&w==0)break;
    x=h*h+w*w;
    max=22500;
    for(i=1;i<151;i++){
      for(j=150;i<j;j--){
	xx=i*i+j*j;
	if((x==xx&&x<max&&h<i)||(x<xx&&xx<max)){
	  max=xx;
	  a1=i;
	  a2=j;
	}	 
	if(max==x)break;
      }
    }
    printf("%d %d\n",a1,a2);
  }
  return 0;
}
 