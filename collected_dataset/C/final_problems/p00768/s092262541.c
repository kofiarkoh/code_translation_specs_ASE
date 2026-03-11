#include<stdio.h>
int main(){
  int m,t,p,R,a,b,c,d,i,j,r[51];
  while(scanf("%d %d %d %d",&m,&t,&p,&R),m||t||p||R){
    int pn[51][11]={0};
    int s[51]={0};
    while(R--){
      scanf("%d %d %d %d",&a,&b,&c,&d);
      if(d)pn[b][c]+=20;
      else s[b]+=100000-a-pn[b][c];
    }
    for(i=1;i<=t;i++)r[i]=i;
    for(i=1;i<=t;i++){
      for(j=i;j>1;j--){
	if(s[r[j-1]]<=s[r[j]]){
	  m=r[j];
	  r[j]=r[j-1];
	  r[j-1]=m;
	}
      }
    }
    for(i=1;i<=t;i++){
      if(i-1){
	if(s[r[i-1]]==s[r[i]])printf("=");
	else                  printf(",");
      }
      printf("%d",r[i]);
    }
    printf("\n");
  }
  return 0;
}