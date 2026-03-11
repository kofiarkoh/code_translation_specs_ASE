#include <stdio.h>
int n;
int main(){

  scanf("%d",&n);
  int arra[n],arrw[n],i , minwr =1001,minwl =1001;

  
  for( i=0;i< n;i++){
    scanf("%d",&arra[i]);
  }

  for( i=0;i< n;i++){
    scanf("%d",&arrw[i]);

  }
  int f = 0;
  for( i=0;i< n;i++){
    if(arra[i]== 0 ){minwr = minwr > arrw[i]? arrw[i] : minwr; f += arra[i];}
    else if(arra[i]== 1 ){minwl = minwl > arrw[i]? arrw[i] : minwl; f += arra[i];}

  }

  if (f == 0 || f == n){printf("0\n");}
  else {
  printf("%d\n",minwr+minwl);
  }
  return 0;
}

