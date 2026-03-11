#include<stdio.h>
int main(){
  int i,j,k=1;
  char s[100010],c[100010];
  scanf("%s %s",s,c);
  for(i=j=0;s[i];i++){
    if(s[i]==c[j])j++;
    if(c[j]==0){
      k++;
      j=0;
    }
  }
  //printf("%d\n",k-1);
  for(i=-1;k;i++)k/=2;
  printf("%d\n",i);
  return 0;
}

