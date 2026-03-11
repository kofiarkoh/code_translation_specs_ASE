#include<stdio.h>

int main(void){
  
  int numlane,numdata,i,swap;

  int lane[30];
  int data[2];

  scanf("%d",&numlane);
  scanf("%d",&numdata);
  
  for(i = 0; i < numlane; i++){
    lane[i] = i + 1;
  }

  for(i = 0; i < numdata; i++){
    scanf("%d,%d",&data[0],&data[1]);
    swap = lane[data[0] - 1];
    lane[data[0] - 1] = lane[data[1] - 1];
    lane[data[1] - 1] = swap;
  }

  for(i = 0; i < numlane; i++){
    printf("%d\n",lane[i]);
  }
  
  return 0;
}