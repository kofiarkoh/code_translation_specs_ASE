#include<stdio.h>
int C[100010]={0};
int blmn(int v,int e,int s,int *fr,int *to,int *co){
  int i,j,f;
  for(i=0;i<v;i++)C[i]=1000000000;
  C[s]=0;
  for(i=0;i<=v;i++){
    for(j=f=0;j<e;j++){
      if(C[to[j]]>C[fr[j]]+co[j]){
	C[to[j]]=C[fr[j]]+co[j];
	f=1;
      }
    }
    if(f==0)break;
  }
  return f;
}
int main(){
  int n,m,s,i,a[100010],b[100010],c[100010];
  scanf("%d %d %d",&n,&m,&s);
  for(i=0;i<m;i++)scanf("%d %d %d",&a[i],&b[i],&c[i]);
  if(blmn(n,m,s,a,b,c))printf("NEGATIVE CYCLE\n");
  else{
    for(i=0;i<n;i++){
      C[i]-1000000000?printf("%d\n",C[i]):printf("INF\n");
    }
  }
  return 0;
}