#include <stdio.h>
#include <string.h>
char tolower(char);//char Large to Small

int main(){
  int i=0,count=0;
  char t,txt[1000]={},word[10]={},end[]={"END_OF_TEXT"};
  char *ret;
  scanf("%s",word);
  for(i=0;i<strlen(word);i++){
    if((int)t>=65&&(int)t<=90){
      t = word[i];
      word[i] = tolower(t);
    }
  }
  i=0;
  scanf("%c",&t);
  while( EOF != scanf("%c",&t)){
    txt[i]=t;
    i++;
    if(strstr(txt,end)!=NULL) break;
  }
  while(1){
    if(strcasestr(txt,word)!=NULL){
      ret = strcasestr(txt,word);

      if(ret-txt<0)return 0;

      txt[ret-txt]='~';
      count++;
    }else{
      break;
    }
  }
    
  printf("%d\n",count);

  return 0;
}

char tolower(char c){
  return (c+32);
}