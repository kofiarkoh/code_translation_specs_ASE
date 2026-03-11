#include<stdio.h>

int main(void){
  int n, m;
  int count = 0;
  scanf("%d %d", &n, &m);

  int tmp;
  if(n > m){
    tmp = n;
    n=m;
    m=tmp;
  }

  if(n==1){
    if(m <= 2){
      count = (n*m)%2;
    }else{
      count = m-2;
    }
  }else if(n==2){
    if(m==2)
      count = 0;
    else
      count = 2*(m-2);
  }else if(n>2){
    count = (n-2)*(m-2);
  }
  
  printf("%d\n", count);
  
  return 0;
}
