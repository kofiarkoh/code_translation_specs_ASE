#include<stdio.h>
#include<string.h>
int main(){
  char s[1010],t[1010];
  scanf("%s %s",s,t);
  int i,j,k,l=strlen(s);
  for(i=0;i<2;i++){
    k=0;//printf("\n");
    for(j=i;j<l;j+=2){//printf("%c ",s[j]);
      for(;t[k]-s[j];k++){
	if(t[k]==0)goto NEXT;
      }
    }
    printf("Yes\n");
    return 0;
  NEXT:;
  }
  printf("No\n");
  return 0;
}
  