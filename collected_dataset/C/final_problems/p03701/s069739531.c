#include<stdio.h>

int main(void){
  int n;
  scanf("%d",&n);

  int s[n];
  for(int i = 0;i < n;i ++){
    scanf("%d",&s[i]);
  }

  int count = 0 , max = 0;
  for(int i = 0;i < (1<<n);i ++){
    int temp = i;
    for(int j = 0;j < n;j ++){
      if(temp%2 == 1){
        count += s[j];
      }
      temp /= 2;
    }
    if(count%10 == 0){
      count = 0;
    }
    if(count > max){
      max = count;
    }
    count = 0;
  }

  printf("%d\n",max);

}