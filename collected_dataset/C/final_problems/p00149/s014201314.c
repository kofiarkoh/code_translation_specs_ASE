#include <stdio.h>

int main(int argc,char* argv[]){
  int i;
  double right,left;
  int eyes[4][2] = {0};
  
  while(scanf("%lf %lf",&right,&left) != EOF){
    if(right >= 1.1){
      eyes[0][0]++;
    }else if(right >= 0.6){
      eyes[1][0]++;
    }else if(right >= 0.2){
      eyes[2][0]++;
    }else{
      eyes[3][0]++;
    }

    if(left >= 1.1){
      eyes[0][1]++;
    }else if(left >= 0.6){
      eyes[1][1]++;
    }else if(left >= 0.2){
      eyes[2][1]++;
    }else{
      eyes[3][1]++;
    }
  }

  for(i = 0; i < 4; i++){
    printf("%d %d\n",eyes[i][0],eyes[i][1]);
  }
  
  return 0;
  
}