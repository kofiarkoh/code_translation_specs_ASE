#include<stdio.h>
int main(){
  int n,c,h,i;
  char s[201];
  scanf("%d",&n);
  while(n--){
    scanf("%s",s);
    if(s[c=0]!='>'||s[h=1]!='\''&&s[1]!='^'){
      printf("NA\n");
      continue;
    }
    for(i=2;s[i];i++){
      if(s[1]=='\''){
	if     (s[i]=='=')c+=h;
	else if(s[i]=='#'){
	  if(h==-1)break;
	  h=-1;
	}
	else if(s[i]=='~'&&s[i+1]==0){
	  if(c!=0)break;
	}
	else break;
      }
      else{
	if(s[i]=='Q'&&s[i+1]=='=')i++;
	else if(s[i]!='~')break;
      }
    }
    if(s[i])printf("NA\n");
    else{
      if(s[1]=='\''){
	if(s[i-2]=='=')printf("A\n");
	else                  printf("NA\n");
      }
      else{
	if(s[i-3]=='=')printf("B\n");
	else           printf("NA\n");
      }
    }
  }
  return 0;
}
	  
	