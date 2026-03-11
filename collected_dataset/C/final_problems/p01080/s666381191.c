#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int main(){
  int n,i,j,a,b,f[100010],q[100010],r,t;
  int ta[100010],to[200010],nt[200010],d[2][100010];
  scanf("%d",&n);
  for(i=0;i<=n;i++)ta[i]=-1;
  for(i=0;i<n-1;i++){
    scanf("%d %d",&a,&b);
    to[i]=b;
    nt[i]=ta[a];
    ta[a]=i;
    to[i+n-1]=a;
    nt[i+n-1]=ta[b];
    ta[b]=i+n-1;
  }
  /*for(i=1;i<=n;i++){
    printf("%d:",i);
    for(j=ta[i];j+1;j=nt[j])printf("%d ",to[j]);printf("\n");
    }//*/
  q[t=0]=1;
  for(i=0;i<=n;i++)f[i]=0;
  for(r=f[q[0]]=1;r-t;t++){//printf("%d->",q[t]);
    for(i=ta[q[t]];i+1;i=nt[i]){
      if(f[to[i]])continue;//printf("%d ",to[i]);
      q[r++]=to[i];
      f[to[i]]=1;
    }//printf("\n");
  }//printf("a\n");

  //printf("\n");
  q[0]=q[r-1];//printf("%d\n",q[0]);
  d[0][q[0]]=t=0;
  for(i=0;i<=n;i++)f[i]=0;
  for(r=f[q[0]]=1;r-t;t++){//printf("%d:%d->",d[0][q[t]],q[t]);
    for(i=ta[q[t]];i+1;i=nt[i]){
      if(f[to[i]])continue;
      q[r++]=to[i];
      f[to[i]]=1;
      d[0][to[i]]=d[0][q[t]]+1;//printf("%d %d:",to[i],d[0][to[i]]);
    }//printf("\n");
  }//printf("a\n");

   //printf("\n"); 
  q[0]=q[r-1];
  d[1][q[0]]=t=0;
  for(i=0;i<=n;i++)f[i]=0;
  for(r=f[q[0]]=1;r-t;t++){//printf("%d:%d->",d[1][q[t]],q[t]);
    for(i=ta[q[t]];i+1;i=nt[i]){
      if(f[to[i]])continue;
      q[r++]=to[i];
      f[to[i]]=1;
      d[1][to[i]]=d[1][q[t]]+1;//printf("%d %d:",to[i],d[1][to[i]]);
    }//printf("\n");
  }//printf("\n");
  //for(i=1;i<=n;i++)printf("%d %d %d\n",i,d[0][i],d[1][i]);
  for(i=1;i<=n;i++)printf("%d\n",n*2-2-MAX(d[0][i],d[1][i]));
  return 0;
}