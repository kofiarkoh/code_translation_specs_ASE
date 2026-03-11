#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
  int q[10000000],t,r,i,c[100010],max=0;
int bfs(int v,int e,int *ta,int *fr,int *to,int *co,int *nt,int s){
  //int q[1000000],t,r,i,c[1010],max=0;
  for(i=0;i<v;i++)c[i]=1000000000;
  c[q[0]=s]=t=0;//printf("a\n");
  for(r=1;r-t;t++){
    for(i=ta[q[t]];i+1;i=nt[i]){//printf("%d %d\n",fr[i],to[i]);
      if(c[to[i]]>c[fr[i]]+co[i]){
	c[to[i]]=c[fr[i]]+co[i];
        max=MAX(max,c[to[i]]);
	q[r++]=to[i];
      }
    }//printf("\n");
  }//printf("g%d\n",*g);
  return max;
}	
int tyokkei(int v,int e,int *a,int *b,int *c){
  int ta[100010],nt[200010],id,i,fr[200010],to[200010],co[200010],max=0;
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i]=ta[a[i]];
    fr[i]=a[i];
    to[i]=b[i];
    co[i]=c[i];
    ta[a[i]]=i;
    nt[i+e]=ta[b[i]];
    fr[i+e]=b[i];
    to[i+e]=a[i];
    co[i+e]=c[i];
    ta[b[i]]=i+e;    
  }//printf("a\n");
  //for(i=0;i<2*e;i++)printf("%d %d\n",fr[i],to[i]);printf("\n");
  for(i=0;i<v;i++)max=MAX(max,bfs(v,e*2,ta,fr,to,co,nt,i));
  return max;
}
int main(){
  int n,a[100010],b[100010],c[100010],i;
  scanf("%d",&n);
  for(i=0;i<n-1;i++)scanf("%d %d %d",&a[i],&b[i],&c[i]);
  printf("%d\n",tyokkei(n,n-1,a,b,c));
  return 0;
}