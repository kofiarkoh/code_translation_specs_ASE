#include<stdio.h>
int main(){
  int i,j,max=0,l;
  char s[100010];
  scanf("%s",s);
  for(l=0;s[l];l++);
  for(i=1;i*2<l-1;i++){
    for(j=0;j<i;j++){
      if(s[j]-s[i+j])break;
    }
    if(i==j)max=i;
  }
  printf("%d\n",max*2);    
  return 0;
}
