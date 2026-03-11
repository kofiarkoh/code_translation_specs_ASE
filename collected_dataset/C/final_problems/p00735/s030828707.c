#include<stdio.h>
#include<math.h>

#define MAX 300000

int main(void){
  
  int i,j;
  char mstable[MAX];
  int ms[MAX];
  int n;

  for(i=0;i<MAX;i++){
    if(i%7 == 1 || i%7 == 6){
      mstable[i] = 1;
    }else{
      mstable[i] = 0;
    }
  }
  mstable[1] = 0;

  j = 6;
  while(j<MAX){
    for(i=2*j;i<MAX;i+=j)mstable[i] = 0;
    while(!mstable[++j]){
      if(j==MAX-1){
	j++;
	break;
      }
    }
  }

  j = 0;
  for(i=0;i<MAX;i++){
    if(mstable[i]){
      ms[j] = i;
      j++;
    }
  }
  
  while(1){
    scanf("%d",&n);
    if(n==1)break;

    printf("%d:",n);
    for(i=0;i<j;i++){
      if(n%ms[i]==0)printf(" %d",ms[i]);
    }
    putchar('\n');
  }

  return 0;
}