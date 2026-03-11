#include<stdio.h>

int main(){
  int count[30], result[30];
  char s[100]="\0";
  scanf("%s", &s);

  for(int j=0; j<30; j++)
    count[j]=0;

  for(int i=0; i<100; i++){
    for(int j=0; j<26; j++){
      if(s[i]=='a'+j){
        count[j]++;
        break;
      }
    }
  }

  for(int j=0; j<26; j++){
    if((count[j]%2)!=0){
      printf("No\n");
      return 0;
      break;
    }
  }

  printf("Yes\n");
  return 0;
}
