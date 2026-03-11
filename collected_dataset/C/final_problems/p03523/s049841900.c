#include<stdio.h>
int main(){
  char i,r=0,s[100],c[]={"AKIHABARA"};
  scanf("%s",s);
  for(i=0;s[i];i++){//printf("%c %c\n",s[i],c[r]);
    if(s[i]==c[r]){
      r++;
      continue;
    }
    if(c[r]=='A'){
      r++;
      i--;
    }
    else break;
  }
  if(c[r]=='A')r++;
  printf("%s\n",s[i]+c[r]?"NO":"YES");
  return 0;
}
