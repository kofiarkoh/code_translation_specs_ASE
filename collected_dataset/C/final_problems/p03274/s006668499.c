#include<stdio.h>

int main(void) {
  int N,K;
  scanf("%d %d",&N,&K);

  int x[N];
  for(int i=0;i<N;i++) scanf("%d",&x[i]);

  int min=100000000;
  for(int i=0;i<N-K+1;i++) {
    int xl,xr;
    xl = x[i];
    xr = x[i+K-1];
    if((xl*xr>=0)&&(xr<=0)) {
      if(-xl<min) min=-xl;
    }
    else if((xl*xr>=0)&&(xl>=0)) {
      if(xr<min) min=xr;
    }
    else {
      long XL,XR;
      XL = xr-2*xl;
      XR = 2*xr-xl;
      if(XL>XR) {
	if(XR<min) min=XR;
      }
      else {
	if(XL<min) min=XL;
      }
    }
  }
  printf("%d",min);
  return(0);
}
      
