#include<stdio.h>
#include <string.h>
int main(){
  int N;
  char s[200000];
  int i;
  int sum=0;
  
  scanf("%d",&N);

  scanf("%s",&s[0]);
 

  
for(i=N-1;i>=0;i--){
if(s[i]=='.')
  if(s[i-1]=='#'){
sum++;

}
  
}

printf("%d",sum);




return 0;
}