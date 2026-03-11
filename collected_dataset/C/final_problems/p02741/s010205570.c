#include <stdio.h>

int main(){
  
  int in[] = {1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1, 4, 1, 51};
  
  int innum = 0;
  
  scanf("%d",&innum);
  
  if(innum > 0 && innum <= 32){
	  printf("%d\n",in[innum-1]);
  }
  return 0;
}
