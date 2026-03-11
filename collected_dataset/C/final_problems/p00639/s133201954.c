#include<stdio.h>
#include<math.h>

int main(void){
  double px,py,vx,vy,D;

  while(1){
    scanf("%lf",&D);
    if(D==0)break;
    scanf("%lf%lf%lf%lf",&px,&py,&vx,&vy);
    
    if(py/px==vy/vx){
      if(fabs(px)<fabs(vx) && ((px>0 && vx>0)||(px<0 && vx<0) )){
	if(2-sqrt(px*px+py*py)<=D)printf("%.8f\n",2-sqrt(px*px+py*py));
	else printf("impossible\n");
	}
      else {
	if(sqrt(px*px+py*py)<=D)printf("%.8f\n",sqrt(px*px+py*py));
	else printf("impossible\n");
	}
    }
    else printf("impossible\n");

  }
  return 0;
}