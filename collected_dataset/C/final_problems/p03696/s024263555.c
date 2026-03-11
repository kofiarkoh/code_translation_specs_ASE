#include<stdio.h>

int main(void){
  int n;
  scanf("%d",&n);
  char s[n];
  scanf("%s",s);

  int count1 = 0 , count2 = 0;
  int ans1 = 0 , ans2 , flag = 0;
  for(int i = 0;i < n;i ++){
    if(s[i] == '('){
      count1 ++;
      flag = 1;
    }
    if(s[i] == ')'){
      count2 ++;
      if(count2 > count1){
        if(flag == 0){
          ans1 ++;
        }
      }
      flag = 0;
    }
  }
  
  ans2 = ans1 + count1 - count2;
  
  char ans[200];
  for(int i = 0;i < ans1;i ++){
    ans[i] = '(';
    
  }
  for(int i =0;i < n;i ++){
    ans[ans1 + i] = s[i];
  }
  for(int i = 0;i < ans2;i ++){
    ans[ans1 + n + i] = ')';
  }
  
  ans[ans1 + ans2 + n] = '\0';

  printf("%s",ans);

  return 0;
}