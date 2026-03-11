#include<stdio.h>
int main(){
  int n,m,f,c;
  char w;
  scanf("%d",&n);
  f=c=0;
  while(n--){
    scanf(" %c %d",&w,&m);
    if(f)continue;
    if(w=='('){
      if(c){
	f=1;
	break;
      }
      c+=m;
    }
    else c-=m;
    if(c<0)f=1;
  }
  if(f||c)printf("NO\n");
  else printf("YES\n"); 
  return 0;
}