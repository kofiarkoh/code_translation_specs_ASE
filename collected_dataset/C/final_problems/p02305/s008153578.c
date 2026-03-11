#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <float.h>

#define Np 1000
#define Nm -1000

double distance(double,double);
double pythagorean(double,double);
int intersection(double,double,double);

double c1x,c1y,c1r;
double c2x,c2y,c2r;

int main(){
  int judge;
  double distance_x,distance_y;
  double dis;
  
  //printf("please enter circle1's x,y,r\n");
  scanf("%lf%lf%lf",&c1x,&c1y,&c1r);
  //printf("please enter circle2's x,y,r\n");
  scanf("%lf%lf%lf",&c2x,&c2y,&c2r);

  if(c1x < Nm || c1x > Np ||
     c1y < Nm || c1y > Np ||
     c2x < Nm || c2x > Np ||
     c2y < Nm || c2y > Np ||
     c1r < 1 || c1r > Np ||
     c2r < 1 || c2r > Np){
    printf("restriction\n");
    printf("-1000 <=c1x,c1y,c2x,c2y<= 1000\n");
    return 0;
  }
  else if(c1x == c2x && c1y == c2y && c1r == c2r){
    printf("restriction\n");
    printf("c1 and c2 not same\n");
    return 0;
  }

  //printf("c1y: %lf\n",c1y);
  //printf("c2y: %lf\n",c2y);

  distance_x = fabs(c1x-c2x);
  distance_y = fabs(c1y-c2y);
  
  //distance_x = distance(c1x,c2x);
  //distance_y = distance(c1y,c1y);
  //printf("%lf",distance_y);

  dis = pythagorean(distance_x,distance_y);
  judge = intersection(dis,c1r,c2r);
  /*
  if(judge == 4){printf("%d common tangent lines\n",judge);return 0;}
  else if(judge == 3){printf("%d common tangent lines\n",judge);return 0;}
  else if(judge == 2){printf("%d common tangent lines\n",judge);return 0;}
  else if(judge == 1){printf("%d common tangent lines\n",judge);return 0;}
  else if(judge == 0){printf("%d common tangent lines\n",judge);return 0;}
  else printf("program error\n");
  */
  if(judge == 4){printf("%d\n",judge);return 0;}
  else if(judge == 3){printf("%d\n",judge);return 0;}
  else if(judge == 2){printf("%d\n",judge);return 0;}
  else if(judge == 1){printf("%d\n",judge);return 0;}
  else if(judge == 0){printf("%d\n",judge);return 0;}
  else printf("program error\n");
}


double distance(double c1,double c2){
  printf("c1: %lf\n",c1);
  printf("c2: %lf\n",c2);
  return fabs(c1-c2);
}

double pythagorean(double a,double b){
  double sum;
  a = pow(a,2.0);
  b = pow(b,2.0);
  sum = a + b;
  //printf("%lf\n",a);
  //printf("%lf\n",b);
  //printf("%lf\n",sum);
  return sqrt(sum);
  
}

int intersection(double dis,double r1,double r2){
  double sum_r;
  double sub_r;
  
  sum_r = r1+r2;
  sub_r = fabs(r1-r2);
  //printf("%lf\n",sum_r);
  //printf("%lf\n",dis);
  if(dis > sum_r)return 4;
  else if(dis == sum_r)return 3;
  else if(sub_r < dis && dis < sum_r)return 2;
  else if(sub_r == dis)return 1;
  else if(dis < sub_r)return 0;
  else {printf("program error\n");return 5;}
}

