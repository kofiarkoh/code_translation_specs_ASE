#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int ta[100010],nt[200010],to[200010],co[200010],g;
int bfs(int v,int s){
  int q[1000000],C[100010], t=0,r,i,max=0;
  for(i=0;i<v;i++)C[i]=1000000000;
  C[q[0]=s]=0;//printf("a\n");
  for(r=1;r-t;t++){
    for(i=ta[q[t]];i+1;i=nt[i]){//printf("%d %d %d\n",q[t],to[i],co[i]);
      if(C[to[i]]>C[q[t]]+co[i])C[q[r++]=to[i]]=C[q[t]]+co[i];
      if(max<C[to[i]])max=C[g=to[i]];//printf("m%d\n",max);
    }
  }
  return max;
}//*/
int tyokkei(int v,int e,int *a,int *b){
  int i,max=0;
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i]=ta[a[i]];
    ta[a[i]]=i;
    nt[i+e]=ta[to[i]];
    to[i+e]=a[i];
    co[i+e]=co[i];
    ta[to[i]]=i+e;    
  }
  bfs(v,0);
  return bfs(v,g);
}
int main(){
  int n,a[100010],b[100010],i;
  scanf("%d",&n);
  for(i=0;i<n-1;i++)scanf("%d %d %d",&a[i],&to[i],&co[i]);
  printf("%d\n",tyokkei(n,n-1,a,b));
  return 0;
}