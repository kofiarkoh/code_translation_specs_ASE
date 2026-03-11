#include <stdio.h>
#include <math.h>

int main(void){
  int x,y,z,tmp,max;

  while(1){
    scanf("%d",&z);
    if(!z) break;

    for(x=z-1,max=0;x>0;x--){
      for(y=x;y>0;y--){
	tmp = pow(x,3) + pow(y,3);
	if(tmp<=pow(z,3)){
	  if(tmp>max) max=tmp;
	  break;
	}
      }
    }
    printf("%d\n",(int)pow(z,3)-max);
  }

  return 0;
}