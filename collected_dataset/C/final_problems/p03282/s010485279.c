#include<stdio.h>
#include<math.h>
int main(){
  char s[110];
  int k,i;
  scanf("%s%d",s,&k);
  while(s[i]=='1'&&i<k-1){
    i++;
  }
  if(k>5000000000000000)
    printf("%c\n",s[i+1]);
  else
    printf("%c\n",s[i]);

  return 0;
}
