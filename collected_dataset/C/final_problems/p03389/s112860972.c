#include<stdio.h>

int dai(int a,int b){
  if(a > b){
    return a;
  }
  else{
    return b;
  }
}

int main(void){
  int a , b , c;
  scanf("%d %d %d",&a ,&b ,&c);

  int max = dai(a,dai(b,c));
  int sa = max - a , sb = max - b , sc = max - c;
  int flag = 1 , ans = 0;
  while(flag == 1){
    if(sa%2 + sb%2 + sc%2 == 2){
      ans += sa/2 + sb/2 + sc/2 + 1;
      flag = 0;
    }
    else if(sa%2 + sb%2 + sc%2 == 0){
      ans += sa/2 + sb/2 + sc/2;
      flag = 0;
    }
    sa ++;
    sb ++;
    sc ++;
  }

  printf("%d\n",ans);

  return 0;
}