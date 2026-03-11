#include <stdio.h>
int abs(int a);
int main(void){
int n,a[100000];
scanf("%d",&n);
for(int i=0;i<n;i++)scanf("%d",a+i);
for(int i=0;i<n;i++){
  int o = 0;
  int p=0;
  for(int j =0;j<n;j++){
    if(i==j)
      continue;
    o+=abs(p-a[j]);
    p=a[j];
  }
  o+=abs(p);
  printf("%d\n",o);
}
}
int abs(int a){
  if(a<0)
    return -1*a;
  else
    return a;
}