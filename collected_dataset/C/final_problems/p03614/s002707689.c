#include<stdio.h>
int main(){
  int n,i,d[100010],s=0;
  scanf("%d",&n);
  for(i=0;i<n;i++)scanf("%d",&d[i]);
  for(i=0;i<n;i++){
    if(d[i]==i+1){
      d[i+1]=d[i];
      s++;
    }
  }
  printf("%d\n",s);
  return 0;
}
