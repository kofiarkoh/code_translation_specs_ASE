#include <stdio.h>

int main() {

  int h,w,x,i,j,minx,mini,minj;

  while(scanf("%d%d",&h,&w),h){

    x = h*h +w*w;
    minx=100000000;
    
    for( i=1;i<100;i++){
      for(j=1;;j++){
	if(i>=j)continue;
	if((i*i+j*j > x) || (i*i+j*j==x && i>h )){
	  if(minx > i*i +j*j){
	    minx=i*i+j*j;
	    mini=i;
	    minj=j;
	  }
	  break;
	}
      }
    }

    printf("%d %d\n",mini,minj);

  }

  return 0;

}