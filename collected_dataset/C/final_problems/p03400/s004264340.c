#include<stdio.h>
int main(){
  int n,d,x,a[101];
  scanf("%d %d %d",&n,&d,&x);
  int i,j;
  for(i=1;i<=n;i++){
    scanf("%d",&a[i]);
    for(j=1;j<=d;j+=a[i]){
      x++;
  }
  }
  printf("%d\n",x);
  return 0;
}

