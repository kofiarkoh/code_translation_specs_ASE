#include<stdio.h>
#include<string.h>
int main(void){
  char s[100001];
  scanf("%s",s);
  long l=strlen(s);
 
  while(1){
    if(strncmp(s+l-5,"dream",5)==0){
      s[l-5]='\0';
      l-=5;
    }else if(strncmp(s+l-7,"dreamer",7)==0){
      s[l-7]='\0';
      l-=7;
    }else if(strncmp(s+l-5,"erase",5)==0){
      s[l-5]='\0';
      l-=5;
    }else if(strncmp(s+l-6,"eraser",6)==0){
      s[l-6]='\0';
      l-=6;
    }else{
      break;
    }
  }
    if(l==0) printf("YES\n");
    else printf("NO\n");
    return 0;
}
 