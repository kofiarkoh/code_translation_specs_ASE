#include<stdio.h>
#include<string.h>
int main(){
  char s[10001];
  char *w;
  while(1){
    fgets(s,10000,stdin);
    if(!strcmp(s,"END OF INPUT\n"))break;
    w=strtok(s," \n");
    printf("%d",strlen(w));    
    while(w=strtok(NULL," \n")){
      printf("%d",strlen(w));
    }
    printf("\n");
  }
  return 0;
}