#include <stdio.h>
#include <string.h>

int main(void){
  int i,n,rst[6];
  float x;

  memset(rst,0,sizeof(rst));

  scanf("%d",&n);
  while(n--){
    scanf("%f",&x);

    if(x<165.0){
      rst[0]++;
    }
    else if(x<170.0){
      rst[1]++;
    }
    else if(x<175.0){
      rst[2]++;
    }
    else if(x<180.0){
      rst[3]++;
    }
    else if(x<185.0){
      rst[4]++;
    }
    else{
      rst[5]++;
    }
  }

  for(i=0;i<6;i++){
    printf("%d:",i+1);
    while(rst[i]--){
      putchar('*');
    }
    putchar('\n');
  }

  return 0;
}