#include<stdio.h>
int main(void)
{
  int n,cnt;
  char s[100];
  cnt=0;
  scanf("%d",&n);
  for(int i=0;i<n;i++){
    scanf("%c",&s[i]);
    if(s[i]=='Y'){
      cnt=1;
    }
  }
  if(cnt==1){
    puts("Four");
  }else{
    puts("Three");
  }
  return 0;
}
