#include<stdio.h>
#include<stdlib.h>
#include<math.h>
 
int calc(float,float,float,float,float,float);

int main(){
  int i;
  float cl[6];
  for(i=0;i<6;i++){
    scanf("%f",&cl[i]);
    /*
    if((i != 2) || (i != 5)){
      if(fabsf (cl[i]) >= 1000.0) return -1;
    }else{
      if(cl[i] < 1.0 || cl[i] > 1000.0) return -2;
    }
    */
  }
  printf("%d\n",calc(cl[0],cl[1],cl[2],cl[3],cl[4],cl[5]));
  return 0;
}

int calc(float c1x,float c1y,float c1r,float c2x,float c2y,float c2r){
  float oto = sqrtf(powf(c1x-c2x,2)+powf(c1y-c2y,2));
  float rtr = c1r + c2r;
  float rir = fabsf(c1r - c2r);
  int f = -1;
  if(oto < rir) f = 0;
  if(oto == rir) f = 1;
  if(rir < oto && oto < rtr) f = 2;
  if(oto == rtr) f = 3;
  if(oto > rtr) f = 4;
  return f;
}

