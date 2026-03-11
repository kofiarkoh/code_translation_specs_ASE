#include<stdio.h>
#include<string.h>

int main(){
  int a,i,n = 0;
  char str[100];
  scanf("%s\n",str);
      
  a = strlen(str);
      
  if(a % 2 == 1){
   for(i=0;i<=((a-1)/2)-1;i++){
    if(str[i] != str[a-i-1]){
      n++;
    }
    }
  }
  else{
    for(i=0;i <= (a/2)-1;i++){
    if(str[i] != str[a-i-1]){n++;}
    }
  }
  printf("%d\n",n);
  return 0;
}
