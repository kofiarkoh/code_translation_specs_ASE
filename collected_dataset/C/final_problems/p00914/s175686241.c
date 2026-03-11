#include<stdio.h>
int a,b,c,s;
void f(int i,int n,int d){
  if(d==b){
    if(n==s)c++;
    return;
  }
  for(i++;i<=a;i++){
    if(n+i>s)break;
    f(i,i+n,d+1);
  }
  return;
}
int main(){
  while(scanf("%d %d %d",&a,&b,&s),a||b||s){
    f(c=0,0,0);
    printf("%d\n",c);
  }
  return 0;
}