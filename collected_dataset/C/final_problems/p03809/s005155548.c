#include<stdio.h>
int d[100010],p[100010],f[100010];
int ta[100010],to[200010],nt[200010];
int g(int s){//printf("%d\n",s);
  f[s]=1;
  int i,n,ha=1;
  for(i=ta[s];i+1;i=nt[i]){
    if(f[to[i]])continue;
    ha=0;
    p[s]+=g(to[i]);
  }
  //if(ha)printf("ha\n");
  if(ha)return d[s];
  n=p[s]-d[s];
  //printf("%d!\n",d[s]-n);
  return d[s]-n;
}
int main(){
  int n,a,b,i;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&d[i+1]);
    ta[i+1]=-1;
  }
  for(i=1;i<n;i++){
    scanf("%d %d",&a,&b);
    to[i]=b;
    nt[i]=ta[a];
    ta[a]=i;
    to[i+n]=a;
    nt[i+n]=ta[b];
    ta[b  ]=i+n;
  }
  for(i=1;i<n;i++){
    if(nt[ta[i]]+1)break;
  }
  printf("%s\n",g(i)?"NO":"YES");
  return 0;
}
